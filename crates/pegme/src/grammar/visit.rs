use crate::grammar::PegTerminal;

use super::{PegExpression, PegRuleName};

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
            PegExpression::Rule(rule) => v.visit_rule(rule),
            PegExpression::Seq(left, right) => v.visit_seq(left, right),
            PegExpression::Choice(left, right) => v.visit_choice(left, right),
            PegExpression::Repetition { expr, min, max } => v.visit_repetition(expr, min, max),
            PegExpression::Predicate { expr, positive } => v.visit_predicate(expr, positive),
            PegExpression::Anything => v.visit_anything(),
            PegExpression::Epsilon => v.visit_epsilon(),
        },
        match expr {
            PegExpression::Terminal(term) => v.visit_terminal_mut(term),
            PegExpression::Rule(rule) => v.visit_rule_mut(rule),
            PegExpression::Seq(left, right) => v.visit_seq_mut(left, right),
            PegExpression::Choice(left, right) => v.visit_choice_mut(left, right),
            PegExpression::Repetition { expr, min, max } => v.visit_repetition_mut(expr, min, max),
            PegExpression::Predicate { expr, positive } => v.visit_predicate_mut(expr, positive),
            PegExpression::Anything => v.visit_anything_mut(),
            PegExpression::Epsilon => v.visit_epsilon_mut(),
        }

    fn visit_terminal, visit_terminal_mut (v, terminal: PegTerminal)
        match terminal {
            PegTerminal::Exact(lit) => v.visit_terminal_exact(lit),
            PegTerminal::CharacterClass(ranges) => v.visit_terminal_ranges(&ranges),
            PegTerminal::PredefinedAscii | PegTerminal::PredefinedUtf8Whitespace | PegTerminal::PredefinedUtf8XidStart | PegTerminal::PredefinedUtf8XidContinue => {},
        },
        match terminal {
            PegTerminal::Exact(lit) => v.visit_terminal_exact_mut(lit),
            PegTerminal::CharacterClass(ranges) => v.visit_terminal_ranges_mut(ranges),
            PegTerminal::PredefinedAscii | PegTerminal::PredefinedUtf8Whitespace | PegTerminal::PredefinedUtf8XidStart | PegTerminal::PredefinedUtf8XidContinue => {},
        }

    fn visit_terminal_exact, visit_terminal_exact_mut (v, lit: &'static str) {}, {}
    fn visit_terminal_ranges, visit_terminal_ranges_mut (v, ranges: Vec<(char, char)>) {}, {}
    fn visit_rule, visit_rule_mut (v, name: PegRuleName) {}, {}
    fn visit_seq, visit_seq_mut (v, left: PegExpression, right: PegExpression)
        { v.visit_expr(left); v.visit_expr(right); },
        { v.visit_expr_mut(left); v.visit_expr_mut(right); }
    fn visit_choice, visit_choice_mut (v, left: PegExpression, right: PegExpression)
        { v.visit_expr(left); v.visit_expr(right); },
        { v.visit_expr_mut(left); v.visit_expr_mut(right); }
    fn visit_repetition, visit_repetition_mut (v, expr: PegExpression, min: u32, max: Option<u32>)
        { v.visit_expr(expr); },
        { v.visit_expr_mut(expr); }
    fn visit_predicate, visit_predicate_mut (v, expr: PegExpression, positive: bool)
        { v.visit_expr(expr); },
        { v.visit_expr_mut(expr); }
    fn visit_anything, visit_anything_mut (v) {}, {}
    fn visit_epsilon, visit_epsilon_mut (v) {}, {}
});
