use std::ops::RangeInclusive;

use super::{PegExpression, PegTerminal};

macro_rules! make_visitors {
    ($visitor_const:ident, $visitor_mut:ident {
        $( fn $visit_const:ident, $visit_mut:ident ($visitor:ident $(, $arg_name:ident : $arg_ty:ty)* $(,)? ) $body_const:expr, $body_mut:expr )*
    }) => {
        pub trait $visitor_const: Sized {
            $(
                fn $visit_const(&mut self, $($arg_name : &$arg_ty),*) {
                    $visit_const(self, $($arg_name,)*);
                }
            )*
        }

        pub trait $visitor_mut: Sized {
            $(
                fn $visit_mut(&mut self, $($arg_name : &mut $arg_ty),*) {
                    $visit_mut(self, $($arg_name,)*);
                }
            )*
        }

        $( #[allow(unused_variables)] pub fn $visit_const($visitor: &mut impl $visitor_const, $($arg_name : &$arg_ty),*) { $body_const } )*
        $( #[allow(unused_variables)] pub fn $visit_mut($visitor: &mut impl $visitor_mut, $($arg_name : &mut $arg_ty),*) { $body_mut } )*
    };
}

make_visitors!(PegExpressionVisitor, PegExpressionVisitorMut {
    fn visit_expr, visit_expr_mut (v, expr: PegExpression)
        match expr {
            PegExpression::Terminal(term) => v.visit_terminal(term),
            PegExpression::NonTerminal { rule_name } => v.visit_rule(rule_name),
            PegExpression::NamedNonTerminal { name, rule_name } => v.visit_named_rule(name, rule_name),
            PegExpression::Seq { left, right } => v.visit_seq(left, right),
            PegExpression::Choice { left, right } => v.visit_choice(left, right),
            PegExpression::Repetition { expr } => v.visit_repetition(expr),
            PegExpression::Predicate { expr, positive } => v.visit_predicate(expr, positive),
        },
        match expr {
            PegExpression::Terminal(term) => v.visit_terminal_mut(term),
            PegExpression::NonTerminal { rule_name } => v.visit_rule_mut(rule_name),
            PegExpression::NamedNonTerminal { name, rule_name } => v.visit_named_rule_mut(name, rule_name),
            PegExpression::Seq { left, right } => v.visit_seq_mut(left, right),
            PegExpression::Choice { left, right } => v.visit_choice_mut(left, right),
            PegExpression::Repetition { expr } => v.visit_repetition_mut(expr),
            PegExpression::Predicate { expr, positive } => v.visit_predicate_mut(expr, positive),
        }

    fn visit_terminal, visit_terminal_mut (v, terminal: PegTerminal)
        match terminal {
            PegTerminal::Epsilon => v.visit_terminal_epsilon(),
            PegTerminal::Any => v.visit_terminal_any(),
            PegTerminal::Literal(lit) => v.visit_terminal_literal(lit),
            PegTerminal::Ranges(ranges) => v.visit_terminal_ranges(&ranges),
        },
        match terminal {
            PegTerminal::Epsilon => v.visit_terminal_epsilon_mut(),
            PegTerminal::Any => v.visit_terminal_any_mut(),
            PegTerminal::Literal(lit) => v.visit_terminal_literal_mut(lit),
            PegTerminal::Ranges(ranges) => v.visit_terminal_ranges_mut(ranges),
        }

    fn visit_terminal_epsilon, visit_terminal_epsilon_mut (v) {}, {}
    fn visit_terminal_any, visit_terminal_any_mut (v) {}, {}
    fn visit_terminal_literal, visit_terminal_literal_mut (v, lit: String) {}, {}
    fn visit_terminal_ranges, visit_terminal_ranges_mut (v, ranges: Vec<RangeInclusive<char>>) {}, {}
    fn visit_rule, visit_rule_mut (v, rule_name: String) {}, {}
    fn visit_named_rule, visit_named_rule_mut (v, name: String, rule_name: String) {}, {}
    fn visit_seq, visit_seq_mut (v, left: PegExpression, right: PegExpression)
        { v.visit_expr(left); v.visit_expr(right); },
        { v.visit_expr_mut(left); v.visit_expr_mut(right); }
    fn visit_choice, visit_choice_mut (v, left: PegExpression, right: PegExpression)
        { v.visit_expr(left); v.visit_expr(right); },
        { v.visit_expr_mut(left); v.visit_expr_mut(right); }
    fn visit_repetition, visit_repetition_mut (v, expr: PegExpression)
        { v.visit_expr(expr); },
        { v.visit_expr_mut(expr); }
    fn visit_predicate, visit_predicate_mut (v, expr: PegExpression, positive: bool)
        { v.visit_expr(expr); },
        { v.visit_expr_mut(expr); }
});
