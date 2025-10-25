use std::fmt::{self, Display, Formatter};

use crate::grammar::PegTerminal;

use super::{PegExpression, PegGrammar, PegRule, PegRuleName};

impl Display for PegGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (name, rule) in &self.rules {
            writeln!(f, "{name}: {rule}")?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for PegRuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for PegRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl PegExpression {
    fn is_atomic(&self) -> bool {
        match self {
            Self::Seq(_, _) => false,
            _ => true,
        }
    }
}

impl Display for PegExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terminal(t) => write!(f, "{t}"),
            Self::Rule(nt) => write!(f, "{}", nt),
            Self::Named(name, e) => {
                write!(f, "{name}@")?;
                match e.is_atomic() {
                    true => write!(f, "{e}"),
                    false => write!(f, "({e})"),
                }
            }
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
            Self::Epsilon => write!(f, "ε"),
        }
    }
}

impl Display for PegTerminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(lit) => {
                write!(f, "\"")?;
                for c in lit.chars() {
                    write_char_escaped(f, c, true)?;
                }
                write!(f, "\"")
            }
            Self::CharacterRanges(ranges) => {
                write!(f, "[")?;
                for &(from, to) in ranges {
                    if from == to {
                        write_char_escaped(f, from, false)?;
                    } else {
                        write_char_escaped(f, from, false)?;
                        write!(f, "-")?;
                        write_char_escaped(f, to, false)?;
                    }
                }
                write!(f, "]")
            }
            Self::PredefinedAscii => write!(f, "[:Ascii:]"),
            Self::PredefinedUtf8Whitespace => write!(f, "[:Utf8Whitespace:]"),
            Self::PredefinedUtf8XidStart => write!(f, "[:Utf8XidStart:]"),
            Self::PredefinedUtf8XidContinue => write!(f, "[:Utf8XidContinue:]"),
        }
    }
}

fn write_char_escaped(f: &mut Formatter, c: char, escape_double_quote: bool) -> fmt::Result {
    if c.is_ascii() {
        match c {
            '\t' => write!(f, "\\t"),
            '\r' => write!(f, "\\r"),
            '\n' => write!(f, "\\n"),
            '\\' => write!(f, "\\\\"),
            '"' if escape_double_quote => write!(f, "\\\""),
            c if c > '\x7e' => write!(f, "\\x{:x}", c as u32),
            c => write!(f, "{c}"),
        }
    } else {
        write!(f, "{c}")
    }
}
