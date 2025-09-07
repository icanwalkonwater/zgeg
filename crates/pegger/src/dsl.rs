use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Add, AddAssign, RangeInclusive, Sub},
};

use crate::{
    visit::{visit_peg_rule, PegExpressionVisitor},
    PegExpression, PegGrammar, PegRule, PegRuleName,
};

const DSL_TRIVIA_RULE_PLACEHOLDER: &str = "@@trivia@@";

#[derive(Default)]
pub struct PegGrammarBuilder {
    names: Vec<PegRuleName>,
    rules: HashMap<PegRuleName, RefCell<Vec<PegExpression>>>,
    trivia_rule_name: Option<PegRuleName>,
}

impl PegGrammarBuilder {
    /// You probably want to use `rules()`.
    pub fn rule<'a>(&'a mut self, name: &'static str) -> PegGrammarRuleBuilder<'a> {
        let name = PegRuleName(name);
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
        let names = names.map(PegRuleName);

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

    fn append_to_rule(&self, name: PegRuleName, expr: PegExpression) {
        self.rules[&name].borrow_mut().push(expr.simplify());
    }

    pub fn set_trivia_rule_name(&mut self, name: &'static str) {
        self.trivia_rule_name = Some(PegRuleName(name));
    }

    pub fn build(mut self) -> PegGrammar {
        assert_eq!(self.names.len(), self.rules.len());
        let mut rules = self
            .names
            .into_iter()
            .map(|name| {
                let v = self.rules.remove(&name).unwrap();
                (name, v)
            })
            .map(|(name, choices)| PegRule::multi(name.0, choices.into_inner()))
            .collect::<Vec<_>>();

        if let Some(trivia_rule) = self.trivia_rule_name {
            // Some trivia rule was provided, replace all occurences of the placeholder one.

            struct TriviaReplacer {
                replace_with: PegRuleName,
            }
            impl PegExpressionVisitor for TriviaReplacer {
                fn visit_rule(&mut self, name: &mut PegRuleName) {
                    if name.0 == DSL_TRIVIA_RULE_PLACEHOLDER {
                        *name = self.replace_with;
                    }
                }
            }
            for r in &mut rules {
                visit_peg_rule(
                    r,
                    &mut TriviaReplacer {
                        replace_with: trivia_rule,
                    },
                );
            }
        } else {
            // If no trivia rule was specified, assume there is no trivia in this grammar.
        }

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
    ($grammar:expr; $($rule:ident),+ $(,)?) => {
        let [$(mut $rule),+] = $grammar.rules([$(stringify!($rule)),+]);
    };
}
pub use setup_rules;

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
            expr: PegExpression::Rule(value.name),
        }
    }
}

/// Convert literal strings to keywords.
impl From<&'static str> for PegExpressionBuilder {
    fn from(value: &'static str) -> Self {
        Self {
            expr: PegExpression::keyword(value),
        }
    }
}

/// Convert inclusive ranges to character groups.
impl From<RangeInclusive<char>> for PegExpressionBuilder {
    fn from(value: RangeInclusive<char>) -> Self {
        Self {
            expr: PegExpression::range(*value.start(), *value.end()),
        }
    }
}

/// Overload add operator to seq operator with trivia.
impl<R: Into<PegExpressionBuilder>> Add<R> for PegExpressionBuilder {
    type Output = Self;
    fn add(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(
                PegExpression::seq(
                    self.expr,
                    PegExpression::optional(PegExpression::rule(DSL_TRIVIA_RULE_PLACEHOLDER)),
                ),
                rhs.into().expr,
            ),
        }
    }
}

/// Overload add operator to seq operator without trivia.
impl<R: Into<PegExpressionBuilder>> Sub<R> for PegExpressionBuilder {
    type Output = Self;
    fn sub(self, rhs: R) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(self.expr, rhs.into().expr),
        }
    }
}

/// Overload add for strings to seq operator.
impl Add<PegExpressionBuilder> for &'static str {
    type Output = PegExpressionBuilder;
    fn add(self, rhs: PegExpressionBuilder) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(
                PegExpression::seq(
                    PegExpression::keyword(self),
                    PegExpression::optional(PegExpression::rule(DSL_TRIVIA_RULE_PLACEHOLDER)),
                ),
                rhs.expr,
            ),
        }
    }
}
impl Sub<PegExpressionBuilder> for &'static str {
    type Output = PegExpressionBuilder;
    fn sub(self, rhs: PegExpressionBuilder) -> Self::Output {
        PegExpressionBuilder {
            expr: PegExpression::seq(PegExpression::keyword(self), rhs.expr),
        }
    }
}

// Builtin literals.

/// Matches nothing without consuming.
pub const EPS: PegExpressionBuilder = PegExpressionBuilder {
    expr: PegExpression::Nothing,
};

/// Matches anything.
pub const ANY: PegExpressionBuilder = PegExpressionBuilder {
    expr: PegExpression::Anything,
};

/// Matches the end of file, shortcut for `!.`, a negative lookahead for anything.
pub fn eof() -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::not_predicate(ANY.expr),
    }
}

pub fn star(expr: impl Into<PegExpressionBuilder>) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::zero_or_more(expr.into().expr),
    }
}

pub fn plus(expr: impl Into<PegExpressionBuilder>) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::one_or_more(expr.into().expr),
    }
}

pub fn opt(expr: impl Into<PegExpressionBuilder>) -> PegExpressionBuilder {
    PegExpressionBuilder {
        expr: PegExpression::optional(expr.into().expr),
    }
}

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
