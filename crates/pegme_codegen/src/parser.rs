use std::collections::HashMap;

use pegme_core::grammar::{Grammar, GrammarRule, PegExpression, PegTerminal};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct ParserCodegenOptions {
    pub pegme_core_import: String,
}

impl Default for ParserCodegenOptions {
    fn default() -> Self {
        Self {
            pegme_core_import: "pegme_core".into(),
        }
    }
}

type RulesIdents = HashMap<String, RuleIdents>;

struct RuleIdents {
    rule: GrammarRule,
    syntax_kind: proc_macro2::Ident,
    parse_fn: proc_macro2::Ident,
    test_fn: proc_macro2::Ident,
}

pub fn parser_for_grammar(
    g: &Grammar,
    parser_name: String,
    start_rule: &str,
    ParserCodegenOptions { pegme_core_import }: ParserCodegenOptions,
) -> String {
    // Generate bunch of identifiers for each rule.
    let idents = g
        .rules()
        .iter()
        .map(|rule| {
            let rule = rule.clone();
            let syntax_kind = format_ident!("{}", rule.name().to_uppercase());
            let parse_fn = format_ident!("parse_{}", rule.name());
            let test_fn = format_ident!("test_{}", rule.name());

            (
                rule.name().to_string(),
                RuleIdents {
                    rule,
                    syntax_kind,
                    parse_fn,
                    test_fn,
                },
            )
        })
        .collect::<RulesIdents>();

    // Idents for the node kind enum.
    let syntax_kind_ident = format_ident!("{parser_name}Kind");
    let parser_ident = format_ident!("{parser_name}Parser");

    // Unzip to make them easier to use in quote!.
    let syntax_kind_variants = idents
        .values()
        .map(|i| i.syntax_kind.clone())
        .collect::<Vec<_>>();
    let parse_fn_idents = idents
        .values()
        .map(|i| i.parse_fn.clone())
        .collect::<Vec<_>>();
    let test_fn_idents = idents
        .values()
        .map(|i| i.test_fn.clone())
        .collect::<Vec<_>>();

    // Generate entry point.

    let entry_point_rule = &idents[start_rule].syntax_kind;
    let entry_point = quote! {
        /// Start parsing the input using the default starting rule.
        pub fn parse(input: impl Into<String>) -> Arc<ConcreteSyntaxTree<#syntax_kind_ident>> {
            parse_rule(input, #syntax_kind_ident::#entry_point_rule)
        }

        /// Alternative entry point to parse the input from any rule.
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

    // Accumulate all functions in this stream.
    let mut parser_body = quote! {};

    for RuleIdents {
        rule,
        syntax_kind,
        parse_fn,
        test_fn,
    } in idents.values()
    {
        let rule_kind = quote! { #syntax_kind_ident::#syntax_kind };

        // Generate the function body.
        let parse_body = codegen_parse_peg_expression(&idents, rule.match_expression());

        // Generate the function boilerplate.
        parser_body.extend(quote! {
            pub(crate) fn #parse_fn(&mut self) {
                let end = self.parser.memo(#rule_kind, self.parser.mark()).unwrap().unwrap();
                let node = self.tree.start_node(#rule_kind);

                #parse_body

                self.parser.reset_to(end);
                self.tree.finish_node(node);
            }
        });

        // Generate the test body.
        let name = rule.name();
        let test_body = codegen_test_peg_expression(
            &idents,
            rule.match_expression(),
            &quote! {{ tracing::trace!("Recognized rule {} at {:?}", #name, self.parser.mark()); }},
            &quote! {{
                self.parser.memoize_miss(#rule_kind, start);
                self.parser.reset_to(start);
                return false;
            }},
        );

        // Generate the test boilerplate.
        parser_body.extend(quote! {
            pub(crate) fn #test_fn(&mut self) -> bool {
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

    // Finalize.

    let pegme_import = quote::format_ident!("{pegme_core_import}");

    let file: syn::File = syn::parse_quote! {
        // DO NOT EDIT.
        // This file is auto-generated.

        use std::sync::Arc;
        use #pegme_import::{packrat::PackratParser, cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder}};

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

fn codegen_parse_peg_expression(idents: &RulesIdents, expr: &PegExpression) -> TokenStream {
    match expr {
        // Epsilon has nothing to parse.
        PegExpression::Terminal(PegTerminal::Epsilon) => quote! {},
        // Just eat a character.
        // We already know it matches.
        PegExpression::Terminal(PegTerminal::Any | PegTerminal::Ranges(_)) => quote! {
            self.tree.push_token(self.parser.anything().unwrap());
        },
        // Eat a literal.
        PegExpression::Terminal(PegTerminal::Literal(lit)) => quote! {
            self.parser.expect(#lit);
            self.tree.push_tokens(#lit);
        },
        PegExpression::NonTerminal { rule_name } => {
            let parse_rule_ident = &idents[rule_name].parse_fn;
            quote! {
                self.#parse_rule_ident();
            }
        }
        PegExpression::NamedNonTerminal { name, rule_name } => {
            // TODO: actually handle the name
            let parse_rule_ident = &idents[rule_name].parse_fn;
            quote! {
                self.#parse_rule_ident();
            }
        }
        PegExpression::Seq { left, right } => {
            let code_left = codegen_parse_peg_expression(idents, left);
            let code_right = codegen_parse_peg_expression(idents, right);
            quote! {
                #code_left
                #code_right
            }
        }
        PegExpression::Choice { left, right } => {
            let test_left = codegen_lookahead_peg_expression(idents, left);
            let code_left = codegen_parse_peg_expression(idents, left);
            let code_right = codegen_parse_peg_expression(idents, right);

            quote! {
                if { #test_left } {
                    #code_left
                } else {
                    #code_right
                }
            }
        }
        PegExpression::Repetition { expr } => {
            let code_test = codegen_lookahead_peg_expression(idents, expr);
            let code_expr = codegen_parse_peg_expression(idents, expr);
            quote! {
                while { #code_test } {
                    #code_expr
                }
            }
        }
        // Predicates don't result in token nodes.
        PegExpression::Predicate { .. } => quote! {},
    }
}

fn codegen_lookahead_peg_expression(idents: &RulesIdents, expr: &PegExpression) -> TokenStream {
    let code_expr = codegen_test_peg_expression(idents, expr, &quote! { true }, &quote! { false });
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
    idents: &RulesIdents,
    expr: &PegExpression,
    fragment_success: &TokenStream,
    fragment_failure: &TokenStream,
) -> TokenStream {
    match expr {
        PegExpression::Terminal(PegTerminal::Epsilon) => quote! { #fragment_success },
        PegExpression::Terminal(PegTerminal::Any) => quote! {
            match self.parser.anything() {
                Some(_) => #fragment_success,
                None => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::Literal(lit)) => quote! {
            match self.parser.expect(#lit) {
                true => #fragment_success,
                false => #fragment_failure,
            }
        },
        PegExpression::Terminal(PegTerminal::Ranges(ranges)) => {
            let (ranges_start, ranges_end) = ranges
                .iter()
                .map(|r| (*r.start(), *r.end()))
                .unzip::<_, _, Vec<_>, Vec<_>>();

            quote! {
                match self.parser.eat(|c| match c {
                    #( #ranges_start ..= #ranges_end => true, )*
                    _ => false,
                }) {
                    Some(_) => #fragment_success,
                    None => #fragment_failure,
                }
            }
        }
        PegExpression::NonTerminal { rule_name }
        | PegExpression::NamedNonTerminal { rule_name, .. } => {
            // NOTE: named expression doesn't mean anything here.

            let test_rule_ident = &idents[rule_name].test_fn;
            quote! {
                match self.#test_rule_ident() {
                    true => #fragment_success,
                    false => #fragment_failure,
                }
            }
        }
        PegExpression::Seq { left, right } => {
            let test_left =
                codegen_test_peg_expression(idents, left, &quote! { true }, &quote! { false });
            let test_right =
                codegen_test_peg_expression(idents, right, &quote! { true }, &quote! { false });

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
        PegExpression::Choice { left, right } => {
            let test_left =
                codegen_test_peg_expression(idents, left, &quote! { true }, &quote! { false });
            let test_right =
                codegen_test_peg_expression(idents, right, fragment_success, fragment_failure);

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
        PegExpression::Repetition { expr } => {
            let test_expr =
                codegen_test_peg_expression(idents, expr, &quote! { true }, &quote! { false });

            quote! {
                while { #test_expr } {}
                #fragment_success
            }
        }
        PegExpression::Predicate { expr, positive } => {
            let test_expr =
                codegen_test_peg_expression(idents, expr, &quote! { true }, &quote! { false });

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
    }
}
