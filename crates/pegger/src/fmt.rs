use std::fmt::Display;

use crate::{PegExpression, PegGrammar, PegRule};

impl Display for PegGrammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in &self.rules {
            writeln!(f, "{}", n)?;
        }
        Ok(())
    }
}

impl Display for PegRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for choice in &self.choices {
            writeln!(f, "  | {choice}")?;
        }
        Ok(())
    }
}

impl Display for PegExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LiteralKeyword(r) => write!(f, "\"{r}\""),
            Self::LiteralRange(from, to) => write!(f, "[{from}-{to}]"),
            Self::Rule(nt) => write!(f, "{}", nt),
            Self::Seq(l, r) => {
                write!(f, "{l} {r}")
            }
            Self::Repetition(e, 0, None) => write!(f, "({e})*"),
            Self::Repetition(e, 1, None) => write!(f, "({e})+"),
            Self::Repetition(e, 0, Some(1)) => write!(f, "({e})?"),
            Self::Repetition(e, min, None) => write!(f, "({e})[{min}:]"),
            Self::Repetition(e, min, Some(max)) => write!(f, "({e})[{min}:{max}]"),
            Self::Predicate(e, true) => write!(f, "&({e})"),
            Self::Predicate(e, false) => write!(f, "!({e})"),
            Self::Anything => write!(f, "."),
            Self::Nothing => write!(f, "ε"),
        }
    }
}
