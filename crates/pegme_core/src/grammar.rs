use std::ops::RangeInclusive;

use itertools::Itertools;

pub mod dsl;
mod fmt;
mod simplify;
mod visit;

pub use simplify::*;
pub use visit::*;

#[derive(Debug, PartialEq, Default)]
pub struct Grammar {
    rules: Vec<GrammarRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRule {
    name: String,
    config: RuleConfig,
    match_expression: PegExpression,
    recovery_expression: Option<PegExpression>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuleConfig {
    pub is_token: bool,
    pub has_ast: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PegExpression {
    Terminal(PegTerminal),
    NonTerminal {
        rule_name: String,
    },
    NamedNonTerminal {
        name: String,
        rule_name: String,
    },
    Seq {
        left: Box<PegExpression>,
        right: Box<PegExpression>,
    },
    Choice {
        left: Box<PegExpression>,
        right: Box<PegExpression>,
    },
    Repetition {
        expr: Box<PegExpression>,
    },
    Predicate {
        positive: bool,
        expr: Box<PegExpression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PegTerminal {
    Any,
    Epsilon,
    Literal(String),
    Ranges(Vec<RangeInclusive<char>>),
}

// === Methods

impl Grammar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rules(rules: Vec<GrammarRule>) -> Result<Self, Box<dyn std::error::Error>> {
        // Find duplicates.
        let err = rules
            .iter()
            .dedup_by_with_count(|a, b| a.name == b.name)
            .filter(|(count, _)| *count > 1)
            .map(|(_, rule)| format!("Rule {} is defined multiple times !", rule.name))
            .join("\n");
        if !err.is_empty() {
            return Err(err)?;
        }

        Ok(Self { rules })
    }

    pub fn rules(&self) -> &[GrammarRule] {
        &self.rules
    }

    pub fn find_rule(&self, name: &str) -> Option<&GrammarRule> {
        self.rules.iter().find(|r| r.name == name)
    }

    pub fn find_rule_mut(&mut self, name: &str) -> Option<&mut GrammarRule> {
        self.rules.iter_mut().find(|r| r.name == name)
    }

    pub fn append_rule(&mut self, rule: GrammarRule) -> Result<(), Box<dyn std::error::Error>> {
        // Check already defined.
        if self.rules.iter().any(|r| r.name == rule.name) {
            return Err(format!("Rule {} already defined", rule.name))?;
        }

        self.rules.push(rule);
        Ok(())
    }
}

impl GrammarRule {
    pub fn new(name: impl Into<String>, expr: PegExpression) -> Self {
        Self {
            name: name.into(),
            config: Default::default(),
            match_expression: expr,
            recovery_expression: None,
        }
    }

    pub fn token(name: impl Into<String>, expr: PegExpression) -> Self {
        Self {
            name: name.into(),
            config: RuleConfig {
                is_token: true,
                ..Default::default()
            },
            match_expression: expr,
            recovery_expression: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> &RuleConfig {
        &self.config
    }

    pub fn match_expression(&self) -> &PegExpression {
        &self.match_expression
    }

    pub fn recovery_expression(&self) -> Option<&PegExpression> {
        self.recovery_expression.as_ref()
    }

    pub fn edit_config(&mut self, edit_fn: impl FnOnce(&mut RuleConfig)) {
        edit_fn(&mut self.config);
    }

    pub fn with_recovery_expression(&mut self, recover: PegExpression) {
        assert!(self.recovery_expression.is_none());
        self.recovery_expression = Some(recover);
    }
}

impl PegExpression {
    pub fn any() -> Self {
        Self::Terminal(PegTerminal::Any)
    }

    pub fn epsilon() -> Self {
        Self::Terminal(PegTerminal::Epsilon)
    }

    pub fn literal(lit: &str) -> Self {
        Self::Terminal(PegTerminal::Literal(lit.into()))
    }

    pub fn ranges(ranges: impl Iterator<Item = RangeInclusive<char>>) -> Self {
        Self::Terminal(PegTerminal::Ranges(ranges.collect()))
    }

    pub fn rule(rule: &str) -> Self {
        Self::NonTerminal {
            rule_name: rule.into(),
        }
    }

    pub fn named_rule(name: &str, rule: &str) -> Self {
        Self::NamedNonTerminal {
            name: name.into(),
            rule_name: rule.into(),
        }
    }
    pub fn seq(self, next: Self) -> Self {
        Self::Seq {
            left: Box::new(self),
            right: Box::new(next),
        }
    }

    pub fn or(self, alt: Self) -> Self {
        Self::Choice {
            left: Box::new(self),
            right: Box::new(alt),
        }
    }

    pub fn star(self) -> Self {
        Self::Repetition {
            expr: Box::new(self),
        }
    }

    pub fn opt(self) -> Self {
        Self::Choice {
            left: Box::new(self),
            right: Box::new(Self::epsilon()),
        }
    }

    pub fn plus(self) -> Self {
        Self::Seq {
            left: Box::new(self.clone()),
            right: Box::new(Self::star(self)),
        }
    }

    pub fn lookahead(self, positive: bool) -> Self {
        Self::Predicate {
            positive,
            expr: Box::new(self),
        }
    }
}
