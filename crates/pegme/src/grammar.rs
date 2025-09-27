pub mod dsl;
mod fmt;
mod simplify;
mod visit;

pub use simplify::*;
pub use visit::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PegGrammar {
    rules: Vec<PegRule>,
}

impl PegGrammar {
    pub fn new(
        rules: impl IntoIterator<Item = PegRule>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let rules = rules.into_iter().collect::<Vec<_>>();

        let mut names = rules.iter().map(|nt| nt.name.0).collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();
        if names.len() != rules.len() {
            return Err(
                "Found duplicated non terminals names, use an ordered choice instead".into(),
            );
        }

        Ok(Self { rules })
    }

    pub fn rule_names(&self) -> Vec<PegRuleName> {
        self.rules.iter().map(|r| r.name).collect()
    }

    pub fn rule_by_name(&self, name: &'static str) -> &PegRule {
        self.rule(PegRuleName(name))
    }

    pub fn rule(&self, name: PegRuleName) -> &PegRule {
        self.rules.iter().find(|r| r.name == name).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PegRuleName(pub(crate) &'static str);

#[derive(Debug, PartialEq, Eq)]
pub struct PegRule {
    name: PegRuleName,
    expr: PegExpression,
}

impl PegRule {
    pub fn simple(name: &'static str, expr: PegExpression) -> Self {
        Self::multi(name, [expr])
    }

    pub fn multi(name: &'static str, choices: impl IntoIterator<Item = PegExpression>) -> Self {
        let mut choices = choices.into_iter();

        if let Some(first) = choices.next() {
            let expr = choices.fold(first, |acc, choice| PegExpression::choice(acc, choice));

            Self {
                name: PegRuleName(name),
                expr,
            }
        } else {
            Self {
                name: PegRuleName(name),
                expr: PegExpression::not_predicate(PegExpression::Epsilon),
            }
        }
    }

    pub fn name(&self) -> PegRuleName {
        self.name
    }

    pub fn expr(&self) -> &PegExpression {
        &self.expr
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PegExpression {
    Terminal(PegTerminal),
    Rule(PegRuleName),
    Seq(Box<PegExpression>, Box<PegExpression>),
    Choice(Box<PegExpression>, Box<PegExpression>),
    Repetition {
        expr: Box<PegExpression>,
        min: u32,
        max: Option<u32>,
    },
    Predicate {
        expr: Box<PegExpression>,
        positive: bool,
    },
    Anything,
    Epsilon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PegTerminal {
    Exact(&'static str),
    CharacterClass(Vec<(char, char)>),
    PredefinedAscii,
    PredefinedUtf8Whitespace,
    PredefinedUtf8XidStart,
    PredefinedUtf8XidContinue,
}

impl PegExpression {
    pub fn exact(kw: &'static str) -> Self {
        Self::Terminal(PegTerminal::Exact(kw))
    }

    pub fn range(from: char, to: char) -> Self {
        Self::Terminal(PegTerminal::CharacterClass(vec![(from, to)]))
    }

    pub fn ranges(ranges: Vec<(char, char)>) -> Self {
        Self::Terminal(PegTerminal::CharacterClass(ranges))
    }

    pub fn any_ascii() -> Self {
        Self::Terminal(PegTerminal::PredefinedAscii)
    }

    pub fn any_utf8_whitespace() -> Self {
        Self::Terminal(PegTerminal::PredefinedUtf8Whitespace)
    }

    pub fn any_utf8_xid_start() -> Self {
        Self::Terminal(PegTerminal::PredefinedUtf8XidStart)
    }

    pub fn any_utf8_xid_continue() -> Self {
        Self::Terminal(PegTerminal::PredefinedUtf8XidContinue)
    }

    pub fn rule(name: &'static str) -> Self {
        Self::Rule(PegRuleName(name))
    }

    pub fn seq(left: impl Into<Box<PegExpression>>, right: impl Into<Box<PegExpression>>) -> Self {
        Self::Seq(left.into(), right.into())
    }

    pub fn choice(
        left: impl Into<Box<PegExpression>>,
        right: impl Into<Box<PegExpression>>,
    ) -> Self {
        Self::Choice(left.into(), right.into())
    }

    pub fn zero_or_more(expr: PegExpression) -> Self {
        Self::Repetition {
            expr: Box::new(expr),
            min: 0,
            max: None,
        }
    }

    pub fn one_or_more(expr: PegExpression) -> Self {
        Self::Repetition {
            expr: Box::new(expr),
            min: 1,
            max: None,
        }
    }

    pub fn optional(expr: PegExpression) -> Self {
        Self::Repetition {
            expr: Box::new(expr),
            min: 0,
            max: Some(1),
        }
    }

    pub fn repetition(expr: PegExpression, min: u32, max: Option<u32>) -> Self {
        Self::Repetition {
            expr: Box::new(expr),
            min,
            max,
        }
    }

    pub fn and_predicate(pred: impl Into<Box<PegExpression>>) -> Self {
        Self::predicate(pred, true)
    }

    pub fn not_predicate(pred: impl Into<Box<PegExpression>>) -> Self {
        Self::predicate(pred, false)
    }

    pub fn predicate(pred: impl Into<Box<PegExpression>>, positive: bool) -> Self {
        Self::Predicate {
            expr: pred.into(),
            positive,
        }
    }

    pub fn anything() -> Self {
        Self::Anything
    }

    pub fn epsilon() -> Self {
        Self::Epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::{PegExpression, PegGrammar, PegRule};

    fn make_simple_adder_ref() -> PegGrammar {
        PegGrammar::new([
            PegRule::simple("root", PegExpression::rule("sum")),
            PegRule::simple(
                "sum",
                PegExpression::seq(
                    PegExpression::rule("value"),
                    PegExpression::zero_or_more(PegExpression::seq(
                        PegExpression::exact("+"),
                        PegExpression::rule("value"),
                    )),
                ),
            ),
            PegRule::multi(
                "value",
                [
                    PegExpression::one_or_more(PegExpression::range('0', '9')),
                    PegExpression::seq(
                        PegExpression::seq(PegExpression::exact("("), PegExpression::rule("sum")),
                        PegExpression::exact(")"),
                    ),
                ],
            ),
        ])
        .unwrap()
    }

    #[test]
    fn simple_adder_fmt() {
        let expected = r#"
root: sum

sum: value ("+" value)*

value: ([0-9]+ / "(" sum ")")
"#;

        let grammar = make_simple_adder_ref();

        pretty_assertions::assert_str_eq!(expected.trim(), grammar.to_string().trim());
    }

    #[test]
    fn simple_adder_dsl() {
        use super::dsl::*;

        let mut grammar = PegGrammarBuilder::default();
        declare_rules!(grammar; root, sum, value);

        root += &sum;
        sum += eps() + &value + (eps() + "+" + &value).star();
        value += (eps() + ['0', '9']).plus();
        value += eps() + "(" + &sum + ")";

        let grammar = grammar.build();

        pretty_assertions::assert_eq!(make_simple_adder_ref(), grammar);
    }
}
