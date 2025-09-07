use std::fmt::Display;

pub mod dsl;
mod fmt;

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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PegRuleName(&'static str);

impl Display for PegRuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PegRule {
    name: PegRuleName,
    choices: Vec<PegExpression>,
}

impl PegRule {
    pub fn simple(name: &'static str, expr: PegExpression) -> Self {
        Self::multi(name, [expr])
    }

    pub fn multi(name: &'static str, choices: impl IntoIterator<Item = PegExpression>) -> Self {
        Self {
            name: PegRuleName(name),
            choices: choices.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PegExpression {
    LiteralKeyword(&'static str),
    LiteralRange(char, char),
    Rule(PegRuleName),
    Seq(Box<PegExpression>, Box<PegExpression>),
    Repetition(Box<PegExpression>, u32, Option<u32>),
    Predicate(Box<PegExpression>, bool),
    Anything,
    Nothing,
}

impl PegExpression {
    pub fn keyword(kw: &'static str) -> Self {
        Self::LiteralKeyword(kw)
    }

    pub fn range(from: char, to: char) -> Self {
        Self::LiteralRange(from, to)
    }

    pub fn rule(name: &'static str) -> Self {
        Self::Rule(PegRuleName(name))
    }

    pub fn seq(left: impl Into<Box<PegExpression>>, right: impl Into<Box<PegExpression>>) -> Self {
        Self::Seq(left.into(), right.into())
    }

    pub fn zero_or_more(expr: PegExpression) -> Self {
        Self::Repetition(Box::new(expr), 0, None)
    }

    pub fn one_or_more(expr: PegExpression) -> Self {
        Self::Repetition(Box::new(expr), 1, None)
    }

    pub fn optional(expr: PegExpression) -> Self {
        Self::Repetition(Box::new(expr), 0, Some(1))
    }

    pub fn repetition(expr: PegExpression, min: u32, max: Option<u32>) -> Self {
        Self::Repetition(Box::new(expr), min, max)
    }

    pub fn and_predicate(pred: impl Into<Box<PegExpression>>) -> Self {
        Self::predicate(pred, true)
    }

    pub fn not_predicate(pred: impl Into<Box<PegExpression>>) -> Self {
        Self::predicate(pred, false)
    }

    pub fn predicate(pred: impl Into<Box<PegExpression>>, positive: bool) -> Self {
        Self::Predicate(pred.into(), positive)
    }

    pub fn anything() -> Self {
        Self::Anything
    }

    pub fn nothing() -> Self {
        Self::Nothing
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Self::Nothing)
    }

    pub fn simplify(self) -> Self {
        match self {
            // These are not simplifyable
            Self::LiteralKeyword(_)
            | Self::LiteralRange(_, _)
            | Self::Rule(_)
            | Self::Anything
            | Self::Nothing => self,
            Self::Seq(l, r) => {
                let l_simplified = l.simplify();
                let r_simplified = r.simplify();
                match (l_simplified, r_simplified) {
                    (Self::Nothing, e) | (e, Self::Nothing) => e,
                    (l, r) => Self::seq(l, r),
                }
            }
            Self::Repetition(expr, min, max) => Self::repetition(expr.simplify(), min, max),
            Self::Predicate(pred, positive) => Self::predicate(pred.simplify(), positive),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{PegExpression, PegGrammar, PegRule};

    fn make_simple_adder_ref() -> PegGrammar {
        PegGrammar::new([
            PegRule::simple("root", PegExpression::rule("sum")),
            PegRule::simple(
                "sum",
                PegExpression::seq(
                    PegExpression::rule("value"),
                    PegExpression::zero_or_more(PegExpression::seq(
                        PegExpression::LiteralKeyword("+"),
                        PegExpression::rule("value"),
                    )),
                ),
            ),
            PegRule::multi(
                "value",
                [
                    PegExpression::one_or_more(PegExpression::LiteralRange('0', '9')),
                    PegExpression::seq(
                        PegExpression::seq(
                            PegExpression::LiteralKeyword("("),
                            PegExpression::rule("sum"),
                        ),
                        PegExpression::keyword(")"),
                    ),
                ],
            ),
        ])
        .unwrap()
    }

    #[test]
    fn simple_adder_fmt() {
        let expected = r#"
root:
  | sum

sum:
  | value ("+" value)*

value:
  | ([0-9])+
  | "(" sum ")"
"#;

        let grammar = make_simple_adder_ref();

        pretty_assertions::assert_str_eq!(expected.trim(), grammar.to_string().trim());
    }

    #[test]
    fn simple_adder_dsl() {
        use crate::dsl::*;

        let mut grammar = PegGrammarBuilder::default();
        setup_rules!(grammar; root, sum, value);

        root += &sum;
        sum += EPS + &value + (EPS + "+" + &value).star();
        value += (EPS + ('0'..='9')).plus();
        value += EPS + "(" + &sum + ")";

        let grammar = grammar.build();

        pretty_assertions::assert_eq!(make_simple_adder_ref(), grammar);
    }
}
