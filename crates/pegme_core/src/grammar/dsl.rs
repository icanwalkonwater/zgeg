use super::*;
use std::ops::{BitOr, RangeInclusive, Sub};

// Helper functions available inside the macro.

pub fn _any() -> PegExpression {
    PegExpression::any()
}

pub fn _eps() -> PegExpression {
    PegExpression::epsilon()
}

pub fn _ranges(ranges: Vec<RangeInclusive<char>>) -> PegExpression {
    PegExpression::ranges(ranges.into_iter())
}

pub fn _named(name: &str, rule: RuleRef) -> PegExpression {
    PegExpression::named_rule(name, rule.0)
}

pub fn _star(expr: impl AsPegExpr) -> PegExpression {
    expr.cast().star()
}

pub fn _opt(expr: impl AsPegExpr) -> PegExpression {
    expr.cast().opt()
}

pub fn _plus(expr: impl AsPegExpr) -> PegExpression {
    expr.cast().plus()
}

pub fn _not(expr: impl AsPegExpr) -> PegExpression {
    expr.cast().lookahead(false)
}

/// Helper trait marking what can be used as an expression.
pub trait AsPegExpr {
    fn cast(&self) -> PegExpression;
}

/// Reflexive impl.
impl AsPegExpr for PegExpression {
    fn cast(&self) -> PegExpression {
        self.clone()
    }
}

/// Strings are literals.
impl AsPegExpr for &'static str {
    fn cast(&self) -> PegExpression {
        PegExpression::literal(self)
    }
}

/// The empty tuple is epsilon.
impl AsPegExpr for () {
    fn cast(&self) -> PegExpression {
        PegExpression::epsilon()
    }
}

/// Operation overloads:
/// - Sub becomes Seq.
/// - BitOr becomes Choice.

impl<T: AsPegExpr> Sub<T> for PegExpression {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self.seq(rhs.cast())
    }
}

impl<T: AsPegExpr> BitOr<T> for PegExpression {
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        self.or(rhs.cast())
    }
}

impl Sub<PegExpression> for &'static str {
    type Output = PegExpression;
    fn sub(self, rhs: PegExpression) -> Self::Output {
        self.cast().seq(rhs)
    }
}
impl Sub<RuleRef> for &'static str {
    type Output = PegExpression;
    fn sub(self, rhs: RuleRef) -> Self::Output {
        self.cast().seq(rhs.cast())
    }
}

impl BitOr<PegExpression> for &'static str {
    type Output = PegExpression;
    fn bitor(self, rhs: PegExpression) -> Self::Output {
        self.cast().or(rhs)
    }
}
impl BitOr<RuleRef> for &'static str {
    type Output = PegExpression;
    fn bitor(self, rhs: RuleRef) -> Self::Output {
        self.cast().or(rhs.cast())
    }
}

/// Handy trivially copyable wrapper to have some type checking in the macro.
#[derive(Clone, Copy)]
pub struct RuleRef(pub &'static str);

impl AsPegExpr for RuleRef {
    fn cast(&self) -> PegExpression {
        PegExpression::rule(self.0)
    }
}

impl<T: AsPegExpr> Sub<T> for RuleRef {
    type Output = PegExpression;
    fn sub(self, rhs: T) -> Self::Output {
        self.cast().seq(rhs.cast())
    }
}

impl<T: AsPegExpr> BitOr<T> for RuleRef {
    type Output = PegExpression;
    fn bitor(self, rhs: T) -> Self::Output {
        self.cast().or(rhs.cast())
    }
}

/// Useful extension to mark a rule as a token after the fact.
impl GrammarRule {
    pub fn with_token(mut self, is_token: bool) -> GrammarRule {
        self.config.is_token = is_token;
        self
    }
}

/// Macro to simplify the declaration of a new pegme grammar.
///
/// # Example
/// ```
/// # use pegme_core::grammar::dsl::*;
/// let g = grammar! {
///   let expr = num - "+" - num;
///   #[token]
///   let num = _plus(_ranges(vec!['0'..='9']));
/// }
/// .unwrap();
/// ```
#[macro_export]
macro_rules! grammar {
    ($(
        $(#[$attr:ident])*
        let $name:ident = $expr:expr;
    )*) => {{
        use $crate::grammar::dsl::*;
        // Predeclare rule refs.
        $(
            #[allow(unused, non_snake_case)]
            let $name = $crate::grammar::dsl::RuleRef(stringify!($name));
        )*

        // Build rules.
        let rules = vec![
            $(
                $crate::grammar::GrammarRule::new(
                    stringify!($name),
                    // Use epsilon here to nudge the type system in the right direction.
                    $crate::grammar::dsl::_eps() - $expr,
                ).with_token($crate::grammar::dsl::grammar!(@is-token $($attr)*)),
            )*
        ];

        // Build grammar.
        $crate::grammar::Grammar::from_rules(rules)
    }};
    (@is-token token) => { true };
    (@is-token) => { false };
}

pub use grammar;
