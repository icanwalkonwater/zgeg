use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Add, AddAssign, BitOr},
};

use crate::grammar::{visit::PegExpressionVisitorMut, PegExpressionSimplifier, PegTerminal};

use super::{PegExpression, PegGrammar, PegRule, PegRuleName};

#[derive(Default)]
pub struct PegGrammarBuilder {
    rules: HashMap<PegRuleName, RefCell<Vec<PegExpression>>>,
}

impl PegGrammarBuilder {
    /// You probably want to use `rules()`.
    pub fn rule<'a>(&'a mut self, name: &'static str) -> PegGrammarRuleBuilder<'a> {
        let name = PegRuleName(name);
        let prev = self.rules.insert(name, RefCell::new(vec![]));
        assert!(prev.is_none());

        PegGrammarRuleBuilder {
            builder: self,
            name,
        }
    }

    pub fn rules<const N: usize>(
        &mut self,
        names: [&'static str; N],
    ) -> [PegGrammarRuleBuilder; N] {
        let names = names.map(PegRuleName);

        for n in names {
            let prev = self.rules.insert(n, RefCell::new(Vec::new()));
            assert!(prev.is_none());
        }
        names.map(|name| PegGrammarRuleBuilder {
            builder: &*self,
            name,
        })
    }

    fn append_to_rule(&self, name: PegRuleName, expr: PegExpression) {
        self.rules[&name].borrow_mut().push(expr);
    }

    pub fn build(self) -> PegGrammar {
        let rules = self
            .rules
            .into_iter()
            .map(|(name, exprs)| {
                let mut rule = PegRule::multi(exprs.into_inner());

                // Simplify rule.
                PegExpressionSimplifier.visit_expr_mut(&mut rule.expr);

                (name, rule)
            })
            .collect();

        PegGrammar::new(rules).unwrap()
    }
}

/// Helper macro prepares multiples rules on a builder.
///
/// # Example
/// ```rust
/// # use pegme::grammar::dsl::*;
/// let mut g = PegGrammarBuilder::default();
/// declare_rules!(g; root, sum, value);
///
/// root += &sum;
/// value += eps() + ['0', '9'];
/// // ...
/// ```
#[macro_export]
macro_rules! declare_rules {
    ($grammar:expr; $($rule:ident),+ $(,)?) => {
        let [$(mut $rule),+] = $grammar.rules([$(stringify!($rule)),+]);
    };
}
pub use declare_rules;

pub struct PegGrammarRuleBuilder<'a> {
    builder: &'a PegGrammarBuilder,
    name: PegRuleName,
}

pub struct PegExpressionBuilder {
    expr: PegExpression,
}

impl PegExpressionBuilder {
    pub fn star(self) -> Self {
        star(self)
    }

    pub fn plus(self) -> Self {
        plus(self)
    }

    pub fn opt(self) -> Self {
        opt(self)
    }
}

// == DSL operators
//
// Those types can be converted to expressions:
// - `&'static str` as exact.
// - `&PegGrammarRuleBuilder` as rule.
// - `[char, char]` as range.
//
// Operators:
// - `+=` to add an alternative to a rule.
// - `+` for sequence.
// - `|` for choice.

pub trait CoercableToPegExpression {
    fn into_expr(self) -> PegExpressionBuilder;
}

impl CoercableToPegExpression for &PegGrammarRuleBuilder<'_> {
    fn into_expr(self) -> PegExpressionBuilder {
        PegExpressionBuilder {
            expr: PegExpression::Rule(self.name),
        }
    }
}
impl CoercableToPegExpression for PegExpressionBuilder {
    fn into_expr(self) -> PegExpressionBuilder {
        self
    }
}
impl CoercableToPegExpression for PegTerminal {
    fn into_expr(self) -> PegExpressionBuilder {
        PegExpressionBuilder {
            expr: PegExpression::Terminal(self),
        }
    }
}
impl CoercableToPegExpression for &'static str {
    fn into_expr(self) -> PegExpressionBuilder {
        PegExpressionBuilder {
            expr: PegExpression::exact(self),
        }
    }
}
impl CoercableToPegExpression for [char; 2] {
    fn into_expr(self) -> PegExpressionBuilder {
        PegExpressionBuilder {
            expr: PegExpression::range(self[0], self[1]),
        }
    }
}

/// Append anything that can be turned into an expression to the grammar rule.
impl<T: CoercableToPegExpression> AddAssign<T> for PegGrammarRuleBuilder<'_> {
    fn add_assign(&mut self, rhs: T) {
        self.builder.append_to_rule(self.name, rhs.into_expr().expr);
    }
}

// Operators for expression builder.

impl<R: CoercableToPegExpression> Add<R> for PegExpressionBuilder {
    type Output = PegExpressionBuilder;
    fn add(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(self.expr, rhs.into_expr().expr),
        }
    }
}
impl<R: CoercableToPegExpression> BitOr<R> for PegExpressionBuilder {
    type Output = PegExpressionBuilder;
    fn bitor(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::choice(self.expr, rhs.into_expr().expr),
        }
    }
}

// Operators for rule builder.

impl<R: CoercableToPegExpression> Add<R> for &PegGrammarRuleBuilder<'_> {
    type Output = PegExpressionBuilder;
    fn add(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(PegExpression::Rule(self.name), rhs.into_expr().expr),
        }
    }
}
impl<R: CoercableToPegExpression> BitOr<R> for &PegGrammarRuleBuilder<'_> {
    type Output = PegExpressionBuilder;
    fn bitor(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::choice(PegExpression::Rule(self.name), rhs.into_expr().expr),
        }
    }
}

// Operators for character class.

impl<R: CoercableToPegExpression> Add<R> for PegTerminal {
    type Output = PegExpressionBuilder;
    fn add(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(PegExpression::Terminal(self), rhs.into_expr().expr),
        }
    }
}
impl<R: CoercableToPegExpression> BitOr<R> for PegTerminal {
    type Output = PegExpressionBuilder;
    fn bitor(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::choice(PegExpression::Terminal(self), rhs.into_expr().expr),
        }
    }
}

// Helper to impl many operators on external types
macro_rules! impl_binop_for_external {
    (impl $op_ty:ident<$rhs:ty> for $lhs:ty, $op_fn:ident, $expr_new:ident) => {
        impl $op_ty<$rhs> for $lhs {
            type Output = PegExpressionBuilder;
            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                PegExpressionBuilder {
                    expr: PegExpression::$expr_new(self.into_expr().expr, rhs.into_expr().expr),
                }
            }
        }
    };
}

// Operators for &'static str.
impl_binop_for_external!(impl Add<PegExpressionBuilder> for &'static str, add, seq);
impl_binop_for_external!(impl Add<&PegGrammarRuleBuilder<'_>> for &'static str, add, seq);
impl_binop_for_external!(impl BitOr<PegExpressionBuilder> for &'static str, bitor, choice);
impl_binop_for_external!(impl BitOr<&PegGrammarRuleBuilder<'_>> for &'static str, bitor, choice);

// Operators for [char; 2].
impl_binop_for_external!(impl Add<PegExpressionBuilder> for [char; 2], add, seq);
impl_binop_for_external!(impl Add<&PegGrammarRuleBuilder<'_>> for [char; 2], add, seq);
impl_binop_for_external!(impl BitOr<PegExpressionBuilder> for [char; 2], bitor, choice);
impl_binop_for_external!(impl BitOr<&PegGrammarRuleBuilder<'_>> for [char; 2], bitor, choice);

// Helpers for common expressions.

// Expose character classes
pub use PegTerminal::{
    PredefinedAscii as Ascii, PredefinedUtf8Whitespace as Utf8Whitespace,
    PredefinedUtf8XidContinue as Utf8XidContinue, PredefinedUtf8XidStart as Utf8XidStart,
};

/// Matches nothing without consuming.
pub fn eps() -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::Epsilon,
    }
}

/// Matches anything.
pub fn any() -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::Anything,
    }
}

pub fn star(expr: impl CoercableToPegExpression) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::zero_or_more(expr.into_expr().expr),
    }
}

pub fn plus(expr: impl CoercableToPegExpression) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::one_or_more(expr.into_expr().expr),
    }
}

pub fn opt(expr: impl CoercableToPegExpression) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::optional(expr.into_expr().expr),
    }
}

pub fn and(expr: impl CoercableToPegExpression) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::and_predicate(expr.into_expr().expr),
    }
}

pub fn not(expr: impl CoercableToPegExpression) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::not_predicate(expr.into_expr().expr),
    }
}

pub fn class(class: &'static str) -> PegExpressionBuilder {
    let mut iter = class.chars().peekable();
    let mut classes = Vec::new();

    // Small parser for character ranges.
    // `[a-z01]` expands to `vec![(a, z), (0, 0), (1, 1)]`.
    loop {
        let Some(c) = iter.next() else {
            break;
        };
        assert_ne!(c, '-');

        if let Some('-') = iter.peek() {
            // Its a range
            iter.next().unwrap();
            let end = iter.next().unwrap();
            classes.push((c, end));
        } else {
            // Its not a range
            classes.push((c, c));
        }
    }

    PegExpressionBuilder {
        expr: PegExpression::ranges(classes),
    }
}
