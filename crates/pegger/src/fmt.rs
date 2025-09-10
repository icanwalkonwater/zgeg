use std::fmt::Display;

use crate::{PegExpression, PegGrammar, PegRule};

impl Display for PegGrammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in &self.rules {
            writeln!(f, "{}", n)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for PegRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)?;
        match self.choices.len() {
            0 => write!(f, " {}", PegExpression::Nothing)?,
            1 => write!(f, " {}", &self.choices[0])?,
            _ => {
                for choice in &self.choices {
                    writeln!(f)?;
                    write!(f, "  | {choice}")?;
                }
            }
        }
        Ok(())
    }
}

impl Display for PegExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LiteralExact(r) => {
                write!(f, "\"")?;
                for c in r.chars() {
                    write_char_escaped(f, c)?;
                }
                write!(f, "\"")
            }
            Self::LiteralRange { from, to } => {
                write!(f, "[")?;
                write_char_escaped(f, *from)?;
                write!(f, "-")?;
                write_char_escaped(f, *to)?;
                write!(f, "]")
            }
            Self::LiteralClass(cls) => write!(f, "[:{cls:?}:]"),
            Self::Rule(nt) => write!(f, "{}", nt),
            Self::Seq(l, r) => write!(f, "{l} {r}"),
            Self::Choice(l, r) => write!(f, "({l} / {r})"),
            Self::Repetition { expr: e, min, max } => {
                match e.is_atomic() {
                    true => write!(f, "{e}")?,
                    false => write!(f, "({e})")?,
                }
                match (min, max) {
                    (0, None) => write!(f, "*")?,
                    (1, None) => write!(f, "+")?,
                    (0, Some(1)) => write!(f, "?")?,
                    (min, None) => write!(f, "[{min}:]")?,
                    (min, Some(max)) => write!(f, "[{min}:{max}]")?,
                }
                Ok(())
            }
            Self::Predicate { expr: e, positive } => {
                match positive {
                    true => write!(f, "&")?,
                    false => write!(f, "!")?,
                }
                match e.is_atomic() {
                    true => write!(f, "{e}")?,
                    false => write!(f, "({e})")?,
                }
                Ok(())
            }
            Self::Anything => write!(f, "."),
            Self::Nothing => write!(f, "ε"),
        }
    }
}

fn write_char_escaped(f: &mut std::fmt::Formatter, c: char) -> std::fmt::Result {
    if c.is_ascii() {
        for cc in std::ascii::escape_default(c as u8) {
            write!(f, "{}", cc as char)?;
        }
        Ok(())
    } else {
        write!(f, "{c}")
    }
}
