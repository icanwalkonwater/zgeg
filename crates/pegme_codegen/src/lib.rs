use pegme_core::grammar::{PegExpression, PegGrammar, PegTerminal};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn parser_for_grammar(g: &PegGrammar, name: String, rule: &str) -> String {
    // Idents for the rule kind enum.
    let syntax_kind_ident = format_ident!("{name}Kind");
    let syntax_kind_variants = g
        .rule_names()
        .iter()
        .map(|name| format_ident!("{}", *name.0))
        .collect::<Vec<_>>();
    let parser_ident = format_ident!("{name}Parser");

    // Idents for the various test/parse functions.
    let parse_fn_idents = g
        .rule_names()
        .iter()
        .map(|name| format_ident!("parse_{}", *name.0))
        .collect::<Vec<_>>();
    let test_fn_idents = g
        .rule_names()
        .iter()
        .map(|name| format_ident!("test_{}", *name.0))
        .collect::<Vec<_>>();

    // Generate entry point.

    let entry_point_rule = format_ident!("{rule}");
    let entry_point = quote! {
        pub fn parse(input: impl Into<String>) -> Arc<ConcreteSyntaxTree<#syntax_kind_ident>> {
            parse_rule(input, #syntax_kind_ident::#entry_point_rule)
        }

        #[inline]
        pub(crate) fn parse_rule(input: impl Into<String>, rule: #syntax_kind_ident) -> Arc<ConcreteSyntaxTree<#syntax_kind_ident>> {
            let mut parser_state = #parser_ident {
                parser: PackratParser::new(input),
                tree: ConcreteSyntaxTreeBuilder::default(),
            };

            match rule {
                #(
                    #syntax_kind_ident::#syntax_kind_variants => {
                        // Prime packrat.
                        let valid = parser_state.#test_fn_idents();
                        assert!(valid, "Couldn't parse {}", #syntax_kind_ident::#syntax_kind_variants);
                        parser_state.parser.reset();

                        parser_state.#parse_fn_idents();
                        parser_state.tree.build()
                    },
                )*
            }
        }
    };

    // Generate enum of syntax kind.

    let syntax_kind_enum = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #syntax_kind_ident {
            #(#syntax_kind_variants,)*
        }

        impl std::fmt::Display for #syntax_kind_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
    };

    // Generate parser state struct.

    let parser_struct = quote! {
        pub(crate) struct #parser_ident {
            pub(crate) parser: PackratParser<#syntax_kind_ident>,
            pub(crate) tree: ConcreteSyntaxTreeBuilder<#syntax_kind_ident>,
        }
    };

    // For each rule:
    // - Generate a parse_x function.
    // - Generate a test_x function.

    let mut parser_body = quote! {};

    for ((rule_name, parse_fn_ident), test_fn_ident) in g
        .rule_names()
        .iter()
        .zip(parse_fn_idents)
        .zip(test_fn_idents)
    {
        let rule = g.rule_by_name(&rule_name.0);

        let doc = format!("{rule}");
        let rule_kind = format_ident!("{rule_name}");
        let rule_kind = quote! { #syntax_kind_ident::#rule_kind };

        let parse_body = codegen_parse_peg_expression(rule.expr());

        parser_body.extend(quote! {
            #[doc = #doc]
            pub(crate) fn #parse_fn_ident(&mut self) {
                let end = self.parser.memo(#rule_kind, self.parser.mark()).unwrap().unwrap();
                let node = self.tree.start_node(#rule_kind);

                #parse_body

                self.parser.reset_to(end);
                self.tree.finish_node(node);
            }
        });

        let name = &*rule_name.0;

        let test_body = codegen_test_peg_expression(
            rule.expr(),
            &quote! {{ tracing::trace!("Recognized rule {} at {:?}", #name, self.parser.mark()); }},
            &quote! {{
                self.parser.memoize_miss(#rule_kind, start);
                self.parser.reset_to(start);
                return false;
            }},
        );

        parser_body.extend(quote! {
            #[doc = #doc]
            pub(crate) fn #test_fn_ident(&mut self) -> bool {
                let start = self.parser.mark();

                match self.parser.memo(#rule_kind, start) {
                    Some(Some(end)) => {
                        self.parser.reset_to(end);
                        return true;
                    }
                    Some(None) => return false,
                    None => {},
                }

                #test_body;

                self.parser.memoize_match(#rule_kind, start, self.parser.mark());
                true
            }
        });
    }

    let file: syn::File = syn::parse_quote! {
        // DO NOT EDIT.
        // This file is auto-generated.

        use std::sync::Arc;
        use pegme_core::{packrat::PackratParser, cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder}};

        #entry_point
        #syntax_kind_enum
        #parser_struct

        #[allow(non_snake_case)]
        impl #parser_ident {
            #parser_body
        }
    };
    prettyplease::unparse(&file)
}

fn codegen_parse_peg_expression(expr: &PegExpression) -> TokenStream {
    match expr {
        PegExpression::Terminal(PegTerminal::Exact(lit)) => quote! {
            self.parser.expect(#lit);
            self.tree.push_tokens(#lit);
        },
        PegExpression::Terminal(
            PegTerminal::CharacterRanges(_)
            | PegTerminal::PredefinedAscii
            | PegTerminal::PredefinedUtf8Whitespace
            | PegTerminal::PredefinedUtf8XidStart
            | PegTerminal::PredefinedUtf8XidContinue,
        ) => quote! {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        },
        PegExpression::Rule(rule) => {
            let parse_rule_ident = format_ident!("parse_{rule}");
            quote! {
                self.#parse_rule_ident();
            }
        }
        PegExpression::Named(_, expr) => {
            // TODO: actually handle the name
            codegen_parse_peg_expression(expr)
        }
        PegExpression::Seq(left, right) => {
            let code_left = codegen_parse_peg_expression(left);
            let code_right = codegen_parse_peg_expression(right);
            quote! {
                #code_left
                #code_right
            }
        }
        PegExpression::Choice(left, right) => {
            let test_left = codegen_lookahead_peg_expression(left);
            let code_left = codegen_parse_peg_expression(left);
            let code_right = codegen_parse_peg_expression(right);

            quote! {
                if { #test_left } {
                    #code_left
                } else {
                    #code_right
                }
            }
        }
        PegExpression::Repetition {
            expr,
            min: 0,
            max: Some(1),
        } => {
            let code_test = codegen_lookahead_peg_expression(expr);
            let code_expr = codegen_parse_peg_expression(expr);

            quote! {
                if { #code_test } {
                    #code_expr
                }
            }
        }
        PegExpression::Repetition {
            expr,
            min: 0,
            max: None,
        } => {
            let code_test = codegen_lookahead_peg_expression(expr);
            let code_expr = codegen_parse_peg_expression(expr);

            quote! {
                while { #code_test } {
                    #code_expr
                }
            }
        }
        PegExpression::Repetition {
            expr,
            min,
            max: None,
        } => {
            let code_test = codegen_lookahead_peg_expression(expr);
            let code_expr = codegen_parse_peg_expression(expr);

            let prefix_code = std::iter::repeat_n(code_expr.clone(), *min as _);

            quote! {
                #( #prefix_code )*

                while { #code_test } {
                    #code_expr
                }
            }
        }
        PegExpression::Repetition {
            expr,
            min,
            max: Some(max),
        } => {
            let code_test = codegen_lookahead_peg_expression(expr);
            let code_expr = codegen_parse_peg_expression(expr);

            let prefix_code = std::iter::repeat_n(code_expr.clone(), *min as _);

            quote! {
                #(#prefix_code)*

                for _ in #min..=#max {
                    if !{ #code_test } {
                        break;
                    }
                    #code_expr
                }
            }
        }
        // Predicates don't result in token nodes.
        PegExpression::Predicate { .. } => quote! {},
        PegExpression::Anything => quote! {
            self.tree.push_token(self.parser.eat(|_| true).unwrap());
        },
        PegExpression::Epsilon => quote! {},
    }
}

fn codegen_lookahead_peg_expression(expr: &PegExpression) -> TokenStream {
    let code_expr = codegen_test_peg_expression(expr, &quote! { true }, &quote! { false });
    quote! {{
        let before_lookahead = self.parser.mark();
        match { #code_expr } {
            true => {
                self.parser.reset_to(before_lookahead);
                true
            },
            false => false,
        }
    }}
}

fn codegen_test_peg_expression(
    expr: &PegExpression,
    fragment_success: &TokenStream,
    fragment_failure: &TokenStream,
) -> TokenStream {
    match expr {
        PegExpression::Terminal(PegTerminal::Exact(lit)) => quote! {
            match self.parser.expect(#lit) {
                true => #fragment_success,
                false => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::CharacterRanges(ranges)) => {
            let (ranges_from, ranges_to) = ranges.iter().copied().unzip::<_, _, Vec<_>, Vec<_>>();
            quote! {
                match self.parser.eat(|c| match c {
                    #( #ranges_from ..= #ranges_to => true, )*
                    _ => false,
                }) {
                    Some(_) => #fragment_success,
                    None => #fragment_failure,
                }
            }
        }
        PegExpression::Terminal(PegTerminal::PredefinedAscii) => quote! {
            match self.parser.eat(|c| c.is_ascii()) {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::PredefinedUtf8Whitespace) => quote! {
            match self.parser.eat(char::is_whitespace) {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::PredefinedUtf8XidStart) => quote! {
            match self.parser.eat(unicode_id_start::is_id_start) {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::PredefinedUtf8XidContinue) => quote! {
            match self.parser.eat(unicode_id_start::is_id_continue) {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Rule(rule) => {
            let test_rule_ident = format_ident!("test_{rule}");
            quote! {
                match self.#test_rule_ident() {
                    true => #fragment_success,
                    false => #fragment_failure,
                }
            }
        }
        PegExpression::Named(_, expr) => {
            // NOTE: named expression doesn't mean anything here.
            codegen_test_peg_expression(expr, fragment_success, fragment_failure)
        }
        PegExpression::Seq(left, right) => {
            let test_left = codegen_test_peg_expression(left, &quote! { true }, &quote! { false });
            let test_right =
                codegen_test_peg_expression(right, &quote! { true }, &quote! { false });

            quote! {
                let before_left = self.parser.mark();
                match { #test_left } {
                    true => match { #test_right } {
                        true => #fragment_success,
                        false => {
                            self.parser.reset_to(before_left);
                            #fragment_failure
                        }
                    },
                    false => #fragment_failure,
                }
            }
        }
        PegExpression::Choice(left, right) => {
            let test_left = codegen_test_peg_expression(left, &quote! { true }, &quote! { false });
            let test_right = codegen_test_peg_expression(right, fragment_success, fragment_failure);

            quote! {
                let before_left = self.parser.mark();
                match { #test_left } {
                    true => #fragment_success,
                    false => {
                        self.parser.reset_to(before_left);
                        #test_right
                    }
                }
            }
        }
        PegExpression::Repetition {
            expr,
            min: 0,
            max: Some(1),
        } => {
            // Special case of the optional operator (`?`).
            // It basically never fails. If the test fails it still passes.

            let test_expr = codegen_test_peg_expression(expr, fragment_success, fragment_success);

            quote! {
                #test_expr
            }
        }
        PegExpression::Repetition {
            expr,
            min: 0,
            max: None,
        } => {
            // Special case of the star operator (`*`).
            // Basically eat as much as it wants. There is no failure

            let test_expr = codegen_test_peg_expression(expr, &quote! { true }, &quote! { false });

            quote! {
                while { #test_expr } {}
                #fragment_success
            }
        }
        PegExpression::Repetition {
            expr,
            min,
            max: None,
        } => {
            assert_ne!(*min, 0);
            // Special case for when there is a min but no max (like `+`).

            let test_expr = codegen_test_peg_expression(expr, &quote! { true }, &quote! { false });
            let test_exprs = std::iter::repeat_n(test_expr.clone(), *min as _).collect::<Vec<_>>();

            quote! {
                let repeat_start = self.parser.mark();
                match true #(&& { #test_exprs })* {
                    true => {
                        while { #test_expr } {}
                        #fragment_success
                    },
                    false => {
                        self.parser.reset_to(repeat_start);
                        #fragment_failure
                    }
                }
            }
        }
        PegExpression::Repetition { expr, min, max } => {
            // This is the general case.
            // It need to keep track of how many matches.

            let test_expr = codegen_test_peg_expression(expr, &quote! { true }, &quote! { false });

            let (loop_check_max, result_check) = match max {
                Some(max) => (
                    quote! {
                        if matches >= #max {
                            break;
                        }
                    },
                    quote! { #min <= matches && matches <= #max },
                ),
                None => (quote! {}, quote! { #min <= matches }),
            };

            quote! {
                let repeat_start = self.parser.mark();
                let mut matches = 0;
                loop {
                    #loop_check_max

                    let loop_start = self.parser.mark();
                    match { #test_expr } {
                        true => matches += 1,
                        false => {
                            self.parser.reset_to(loop_start);
                            break;
                        }
                    }
                }

                match #result_check {
                    true => #fragment_success,
                    false => {
                        self.parser.reset_to(repeat_start);
                        #fragment_failure
                    }
                }
            }
        }
        PegExpression::Predicate { expr, positive } => {
            let test_expr = codegen_test_peg_expression(expr, &quote! { true }, &quote! { false });

            quote! {
                let before_predicate = self.parser.mark();
                let predicate = { #test_expr };
                self.parser.reset_to(before_predicate);
                match predicate == #positive {
                    true => #fragment_success,
                    false => #fragment_failure,
                }
            }
        }
        PegExpression::Anything => quote! {
            match self.parser.eat(|_| true) {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Epsilon => quote! { #fragment_success },
    }
}
