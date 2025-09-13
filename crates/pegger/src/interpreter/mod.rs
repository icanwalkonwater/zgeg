use std::time::Duration;

use crate::{
    grammar::{PegCharacterClass, PegExpression, PegGrammar, PegRuleName},
    packrat::PackratParser,
};

struct PegInterpreterState<'g, 'p> {
    grammar: &'g PegGrammar,
    parser: &'p mut PackratParser,
}

pub fn parse_with_grammar(g: &PegGrammar, rule: &'static str, input: impl Into<String>) -> bool {
    let mut parser = PackratParser::new(input);

    let mut state = PegInterpreterState {
        grammar: g,
        parser: &mut parser,
    };
    state.eval_nonterminal(PegRuleName(rule))
}

impl<'g, 'p> PegInterpreterState<'g, 'p> {
    fn eval_nonterminal(&mut self, rule: PegRuleName) -> bool {
        let mark = self.parser.mark();
        if let Some(v) = self.parser.memo(rule.0, mark) {
            println!("Eval {rule} at {} (memo)", mark.offset());
            return v;
        }
        println!("Eval {rule} at {}", mark.offset());

        let mut res = false;
        for choice in self.grammar.rule(rule).choices() {
            if self.eval_expression(choice) {
                // Stop at the first match
                res = true;
                break;
            }
            self.parser.reset_to(mark);
        }

        self.parser.memoize(rule.0, mark, res);
        res
    }

    fn eval_expression(&mut self, expr: &PegExpression) -> bool {
        let mark = self.parser.mark();
        let matches = match expr {
            PegExpression::LiteralExact(lit) => self.parser.expect(lit),
            PegExpression::LiteralRange { from, to } => {
                self.parser.eat().filter(|c| from <= c && c <= to).is_some()
            }
            PegExpression::LiteralClass(class) => {
                if let Some(c) = self.parser.eat() {
                    match class {
                        PegCharacterClass::UserDefined(ranges) => {
                            ranges.iter().any(|[from, to]| *from <= c && c <= *to)
                        }
                        PegCharacterClass::Ascii => c.is_ascii(),
                        PegCharacterClass::Utf8Whitespace => c.is_whitespace(),
                        PegCharacterClass::Utf8XidStart => unicode_id_start::is_id_start(c),
                        PegCharacterClass::Utf8XidContinue => unicode_id_start::is_id_continue(c),
                    }
                } else {
                    false
                }
            }
            PegExpression::Rule(nt) => self.eval_nonterminal(*nt),
            PegExpression::Seq(first, second) => {
                self.eval_expression(first) && self.eval_expression(second)
            }
            PegExpression::Choice(first, second) => {
                if !self.eval_expression(first) {
                    self.parser.reset_to(mark);
                    self.eval_expression(second)
                } else {
                    true
                }
            }
            PegExpression::Repetition { expr, min, max } => {
                let max = max.unwrap_or(u32::MAX);

                // Greedily match as much as possible.
                let mut matches = 0;
                while matches < max {
                    let mark = self.parser.mark();
                    if !self.eval_expression(expr) {
                        // Backtrack the failed match.
                        self.parser.reset_to(mark);
                        break;
                    }
                    matches += 1;
                }

                *min <= matches && matches <= max
            }
            PegExpression::Predicate { expr, positive } => {
                let mark = self.parser.mark();
                let matches = self.eval_expression(expr);
                self.parser.reset_to(mark);

                // scary shit
                //   - bitsneak (probably)
                if *positive {
                    matches
                } else {
                    !matches
                }
            }
            PegExpression::Anything => self.parser.eat().is_some(),
            PegExpression::Nothing => true,
        };

        if !matches {
            self.parser.reset_to(mark);
        }
        matches
    }
}
