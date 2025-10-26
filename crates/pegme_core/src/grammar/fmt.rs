use std::fmt::{self, Display, Formatter};

use super::*;

impl Display for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in self.rules() {
            writeln!(f, "{r}")?;
        }
        Ok(())
    }
}

impl Display for GrammarRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.config.is_token {
            write!(f, "token ")?;
        } else {
            write!(f, "rule ")?;
        }
        write!(f, "{} = {};", self.name(), self.match_expression())
    }
}

impl Display for PegExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terminal(PegTerminal::Epsilon) => write!(f, "\\e"),
            Self::Terminal(PegTerminal::Any) => write!(f, "."),
            Self::Terminal(PegTerminal::Literal(lit)) => {
                write!(f, "\"")?;
                for c in lit.chars() {
                    write_char_escaped(f, c, true)?;
                }
                write!(f, "\"")
            }
            Self::Terminal(PegTerminal::Ranges(ranges)) => {
                write!(f, "[")?;
                for r in ranges {
                    write_char_escaped(f, *r.start(), false)?;
                    if r.start() != r.end() {
                        write!(f, "-")?;
                        write_char_escaped(f, *r.end(), false)?;
                    }
                }
                write!(f, "]")
            }
            Self::NonTerminal { rule_name } => write!(f, "{rule_name}"),
            Self::NamedNonTerminal { name, rule_name } => write!(f, "{name}@{rule_name}"),
            Self::Seq { left, right } => write!(f, "{left} {right}"),
            Self::Choice { left, right } => write!(f, "({left}) / ({right})"),
            Self::Repetition { expr } => write!(f, "({expr})*"),
            Self::Predicate {
                positive: true,
                expr,
            } => write!(f, "&({expr})"),
            Self::Predicate {
                positive: false,
                expr,
                ..
            } => write!(f, "!({expr})"),
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
