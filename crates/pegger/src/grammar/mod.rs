pub mod dsl;
mod fmt;
mod visit;

use visit::*;

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

    pub fn name(&self) -> PegRuleName {
        self.name
    }

    pub fn choices(&self) -> &[PegExpression] {
        &self.choices
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PegExpression {
    LiteralExact(&'static str),
    LiteralRange {
        from: char,
        to: char,
    },
    LiteralClass(PegCharacterClass),
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
    Nothing,
}

impl PegExpression {
    pub fn exact(kw: &'static str) -> Self {
        Self::LiteralExact(kw)
    }

    pub fn range(from: char, to: char) -> Self {
        Self::LiteralRange { from, to }
    }

    pub fn class(class: PegCharacterClass) -> Self {
        Self::LiteralClass(class)
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

    pub fn nothing() -> Self {
        Self::Nothing
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Self::Nothing)
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Self::Seq(_, _) | Self::Choice(_, _) => false,
            _ => true,
        }
    }

    pub fn simplify(self) -> Self {
        match self {
            // These are not simplifyable
            Self::LiteralExact(_)
            | Self::LiteralRange { from: _, to: _ }
            | Self::LiteralClass(_)
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
            Self::Choice(l, r) => Self::choice(l.simplify(), r.simplify()),
            Self::Repetition { expr, min, max } => Self::repetition(expr.simplify(), min, max),
            Self::Predicate {
                expr: pred,
                positive,
            } => Self::predicate(pred.simplify(), positive),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PegCharacterClass {
    UserDefined(Vec<[char; 2]>),
    Ascii,
    Utf8Whitespace,
    Utf8XidStart,
    Utf8XidContinue,
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
                        PegExpression::LiteralExact("+"),
                        PegExpression::rule("value"),
                    )),
                ),
            ),
            PegRule::multi(
                "value",
                [
                    PegExpression::one_or_more(PegExpression::LiteralRange { from: '0', to: '9' }),
                    PegExpression::seq(
                        PegExpression::seq(
                            PegExpression::LiteralExact("("),
                            PegExpression::rule("sum"),
                        ),
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

value:
  | [0-9]+
  | "(" sum ")"
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
