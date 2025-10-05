use crate::grammar::{PegExpression, PegGrammar, PegRule, PegTerminal};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

pub fn parser_for_grammar(g: &PegGrammar, name: String) -> String {
    let syntax_kind_ident = format_ident!("{name}Kind");
    let syntax_kind_variants = g
        .rule_names()
        .iter()
        .map(|name| format_ident!("{}", name.0))
        .collect::<Vec<_>>();
    let parser_ident = format_ident!("{name}Parser");

    // Generate entry point.

    let entry_point = quote! {
        pub fn parse(input: String) -> Arc<ConcreteSyntaxTree<#syntax_kind_ident>> {
            todo!()
        }
    };

    // Generate enum of syntax kind.

    let syntax_kind_enum = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #syntax_kind_ident {
            #(#syntax_kind_variants,)*
        }
    };

    // Generate parser state struct.

    let parser_struct = quote! {
        pub struct #parser_ident {
            parser: PackratParser<#syntax_kind_ident>,
            tree: ConcreteSyntaxTree<#syntax_kind_ident>,
        }
    };

    // For each rule:
    // - Generate a parse_x function.
    // - Generate a test_x function.

    let mut parser_body = quote! {};

    for rule_name in g.rule_names() {
        let rule = g.rule_by_name(rule_name.0);

        let doc = format!("{rule}");
        let parse_fn_ident = format_ident!("parse_{rule_name}");
        let test_fn_ident = format_ident!("test_{rule_name}");

        let rule_kind = format_ident!("{rule_name}");
        let rule_kind = quote! { #syntax_kind_ident::#rule_kind };

        parser_body.extend(quote! {
            #[doc = #doc]
            fn #parse_fn_ident(&mut self) { todo!() }
        });

        let test_body = codegen_test_peg_expression(
            rule.expr(),
            &quote! {{}},
            &quote! {{
                self.parser.memoize_miss(#rule_kind, start);
                self.parser.reset_to(start);
                return false;
            }},
        );

        parser_body.extend(quote! {
            #[doc = #doc]
            fn #test_fn_ident(&mut self) -> bool {
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

    quote! {
        use std::sync::Arc;
        use crate::{packrat::PackratParser, cst::ConcreteSyntaxTree};

        #entry_point
        #syntax_kind_enum
        #parser_struct

        #[allow(non_snake_case)]
        impl #parser_ident {
            #parser_body
        }
    }
    .to_string()
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
            let test_right = codegen_test_peg_expression(right, fragment_success, fragment_failure);
            quote! {
                match { #test_left } {
                    true => {
                        #test_right
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
            let test_expr = codegen_test_peg_expression(expr, &fragment_success, &fragment_success);

            quote! {
                #test_expr
            }
        }
        PegExpression::Repetition { expr, min, max } => {
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
