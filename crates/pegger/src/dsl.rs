use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Add, AddAssign, RangeInclusive},
};

use crate::{PegExpression, PegGrammar, PegRule};

#[derive(Default)]
pub struct PegGrammarBuilder {
    names: Vec<&'static str>,
    rules: HashMap<&'static str, RefCell<Vec<PegExpression>>>,
}

impl PegGrammarBuilder {
    /// You probably want to use `rules()`.
    pub fn rule<'a>(&'a mut self, name: &'static str) -> PegGrammarRuleBuilder<'a> {
        let prev = self.rules.insert(name, RefCell::new(vec![]));
        assert!(prev.is_none());
        self.names.push(name);

        PegGrammarRuleBuilder {
            builder: self,
            name,
        }
    }

    pub fn rules<const N: usize>(
        &mut self,
        names: [&'static str; N],
    ) -> [PegGrammarRuleBuilder; N] {
        for n in names {
            let prev = self.rules.insert(n, RefCell::new(Vec::new()));
            assert!(prev.is_none());
            self.names.push(n);
        }
        names.map(|name| PegGrammarRuleBuilder {
            builder: &*self,
            name,
        })
    }

    fn append_to_rule(&self, name: &'static str, expr: PegExpression) {
        self.rules[name].borrow_mut().push(expr.simplify());
    }

    pub fn build(mut self) -> PegGrammar {
        assert_eq!(self.names.len(), self.rules.len());
        let rules = self
            .names
            .into_iter()
            .map(|name| {
                let v = self.rules.remove(name).unwrap();
                (name, v)
            })
            .map(|(name, choices)| PegRule::multi(name, choices.into_inner()))
            .collect::<Vec<_>>();
        PegGrammar::new(rules).unwrap()
    }
}

/// Helper macro prepares multiples rules on a builder.
///
/// # Example
/// ```rust
/// # use pegger::dsl::*;
/// let mut g = PegGrammarBuilder::default();
/// setup_rules!(g; root, sum, value);
///
/// root += &sum;
/// value += EPS + ('0'..='9');
/// // ...
/// ```
#[macro_export]
macro_rules! setup_rules {
    ($grammar:expr; $($rule:ident),+) => {
        let [$(mut $rule),+] = $grammar.rules([$(stringify!($rule)),+]);
    };
}
pub use setup_rules;

pub struct PegGrammarRuleBuilder<'a> {
    builder: &'a PegGrammarBuilder,
    name: &'static str,
}

pub struct PegExpressionBuilder {
    expr: PegExpression,
}

impl PegExpressionBuilder {
    pub fn star(self) -> Self {
        let Self { expr } = self;
        Self {
            expr: PegExpression::zero_or_more(expr),
        }
    }

    pub fn plus(self) -> Self {
        let Self { expr } = self;
        Self {
            expr: PegExpression::one_or_more(expr),
        }
    }
}

/// Append anything that can be turnedinto an expression to the grammar rule.
impl<T: Into<PegExpressionBuilder>> AddAssign<T> for PegGrammarRuleBuilder<'_> {
    fn add_assign(&mut self, rhs: T) {
        self.builder.append_to_rule(self.name, rhs.into().expr);
    }
}

/// Convert rule builder into a rule expression.
impl From<&PegGrammarRuleBuilder<'_>> for PegExpressionBuilder {
    fn from(value: &PegGrammarRuleBuilder) -> Self {
        Self {
            expr: PegExpression::rule(value.name),
        }
    }
}

/// Convert literal strings to keywords
impl From<&'static str> for PegExpressionBuilder {
    fn from(value: &'static str) -> Self {
        Self {
            expr: PegExpression::keyword(value),
        }
    }
}

/// Convert inclusive ranges to character groups
impl From<RangeInclusive<char>> for PegExpressionBuilder {
    fn from(value: RangeInclusive<char>) -> Self {
        Self {
            expr: PegExpression::range(*value.start(), *value.end()),
        }
    }
}

/// Overload add operator to sequence operator
impl<R: Into<PegExpressionBuilder>> Add<R> for PegExpressionBuilder {
    type Output = Self;
    fn add(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(self.expr, rhs.into().expr),
        }
    }
}

// Builtin literals.

/// Matches nothing, useful for
pub const EPS: PegExpressionBuilder = PegExpressionBuilder {
    expr: PegExpression::Nothing,
};

pub fn and(expr: impl Into<PegExpressionBuilder>) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::and_predicate(expr.into().expr),
    }
}

pub fn not(expr: impl Into<PegExpressionBuilder>) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::not_predicate(expr.into().expr),
    }
}
