use std::sync::Arc;

use itertools::Itertools;

use crate::{
    cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder},
    grammar::{PegExpression, PegGrammar, PegRuleName, PegTerminal},
    packrat::{PackratMark, PackratParser},
};

pub fn parse_with_grammar(
    g: &PegGrammar,
    root: &'static str,
    input: String,
) -> Option<Arc<ConcreteSyntaxTree<&'static str>>> {
    let mut state = InterpreterState {
        grammar: g,
        parser: PackratParser::new(input),
        tree: ConcreteSyntaxTreeBuilder::default(),
    };

    // This also initializes the packrat memos.
    let matches = state.test_expression(&PegExpression::Rule(PegRuleName(root)));

    if !matches {
        println!("No match");
        return None;
    }

    // Build the CST.
    state.parse_rule(PegRuleName(root));

    Some(state.tree.build())
}

struct InterpreterState<'g> {
    grammar: &'g PegGrammar,
    parser: PackratParser<PegRuleName, ()>,
    tree: ConcreteSyntaxTreeBuilder<&'static str>,
}

#[derive(Debug)]
struct ScavengeReport {
    named_nodes: Vec<(PegRuleName, PackratMark, PackratMark)>,
}

impl InterpreterState<'_> {
    /// This should only be called on rules that we know match.
    ///
    /// Builds the concrete syntax tree for this rule, called recursively.
    fn parse_rule(&mut self, name: PegRuleName) {
        let start = self.parser.mark();
        let (end, _) = self.parser.memo(name, start).unwrap().unwrap();

        let report = self.scavenge_rule(name);
        dbg!(&report);

        assert!(
            report.named_nodes.iter().all(|(_, s, e)| s <= e),
            "Scavenger returned nonsensical marks"
        );
        for ((_, _, left), (_, right, _)) in report.named_nodes.iter().tuple_windows() {
            assert!(
                left <= right,
                "Scout return marks that aren't consecutive: {left:?} > {right:?}"
            );
        }

        self.parser.reset_to(start);

        self.tree.start_node(name.0);

        for (rule, start, end) in report.named_nodes {
            // Eat leading tokens
            let tokens = self.parser.eat_up_to(start);
            self.tree.push_tokens(tokens);

            self.parse_rule(rule);
            assert_eq!(self.parser.mark(), end);
        }

        // Eat trailing tokens
        let tokens = self.parser.eat_up_to(end);
        self.tree.push_tokens(tokens);

        self.tree.finish_node();
    }

    /// This should only be called on rules that we know match.
    ///
    /// Scaffolding for `scavenge_expression`.
    fn scavenge_rule(&mut self, rule: PegRuleName) -> ScavengeReport {
        assert!(
            matches!(self.parser.memo(rule, self.parser.mark()), Some(Some(_))),
            "Tried to scavenge a rule that doesn't match"
        );

        let mut report = ScavengeReport {
            named_nodes: Default::default(),
        };

        let rule = self.grammar.rule(rule);
        self.scavenge_expression(rule.expr(), &mut report);

        report
    }

    /// This should only be called on expression that we know match.
    ///
    /// Walks the current expression to collect the matching non terminals and remember their
    /// positions, called recursively.
    fn scavenge_expression(&mut self, expr: &PegExpression, report: &mut ScavengeReport) {
        {
            // Sanity check
            let start = self.parser.mark();
            assert!(
                self.test_expression(expr),
                "Tried to scavenge an expression that doesn't match"
            );
            self.parser.reset_to(start);
        }

        use PegExpression::*;
        match expr {
            // Just advance terminals, there is nothing to scavenge there.
            expr @ Terminal(_) => {
                self.test_expression(expr);
            }
            // This is the main thing we are looking for.
            Rule(name) => {
                let start = self.parser.mark();

                let (end, _) = self.parser.memo(*name, start).unwrap().unwrap();
                report.named_nodes.push((*name, start, end));
            }
            Named(_, expr) => self.scavenge_expression(expr, report),
            Seq(l, r) => {
                self.scavenge_expression(l, report);
                self.scavenge_expression(r, report);
            }
            Choice(l, r) => {
                // We don't know if the first half matches so we need to do a bit of gymnastic to
                // figure it out.

                let start = self.parser.mark();
                if self.test_expression(l) {
                    // It's the first choice.
                    self.parser.reset_to(start);

                    self.scavenge_expression(l, report);
                } else {
                    // It's the second choice.
                    // No rollback since the test didn't pass and didn't consume.
                    self.scavenge_expression(r, report);
                }
            }
            Repetition { expr, min, max } => {
                // This one is tricky, we don't know how many are supposed to match.
                // So we basically do it all over again.

                let max = max.unwrap_or(u32::MAX);
                let mut matches = 0;

                // Fast track the `min` first.
                while matches < *min {
                    self.scavenge_expression(expr, report);
                }

                while matches < max {
                    let start = self.parser.mark();
                    if self.test_expression(expr) {
                        self.parser.reset_to(start);
                        self.scavenge_expression(expr, report);
                        matches += 1;
                    } else {
                        break;
                    }
                }
            }
            Predicate { .. } => {
                // This is just a noop, we know it matches and there is nothing to scavenge.
            }
            Anything => {
                self.parser.anything();
            }
            Epsilon => {}
        }
    }

    /// Implements the PEG operators, called recursively.
    fn test_expression(&mut self, expr: &PegExpression) -> bool {
        use PegExpression::*;
        match expr {
            Terminal(PegTerminal::Exact(lit)) => self.parser.expect(lit),
            Terminal(PegTerminal::CharacterRanges(ranges)) => self
                .parser
                .eat(|c| ranges.iter().any(|&(from, to)| from <= c && c <= to))
                .is_some(),
            Terminal(PegTerminal::PredefinedAscii) => self.parser.eat(|c| c.is_ascii()).is_some(),
            Terminal(PegTerminal::PredefinedUtf8Whitespace) => {
                self.parser.eat(char::is_whitespace).is_some()
            }
            Terminal(PegTerminal::PredefinedUtf8XidStart) => {
                self.parser.eat(unicode_id_start::is_id_start).is_some()
            }
            Terminal(PegTerminal::PredefinedUtf8XidContinue) => {
                self.parser.eat(unicode_id_start::is_id_continue).is_some()
            }
            Rule(name) => {
                let start = self.parser.mark();

                // Look up the rule's memo and only test it if it doesn't pass.
                match self.parser.memo(*name, start) {
                    Some(Some(_)) => true,
                    Some(None) => false,
                    None => {
                        let rule = self.grammar.rule(*name);
                        let matches = self.test_expression(rule.expr());

                        if matches {
                            println!("Memoize match {name} at {start:?}");
                            self.parser
                                .memoize_match(*name, start, self.parser.mark(), ());
                            true
                        } else {
                            self.parser.memoize_miss(*name, start);
                            false
                        }
                    }
                }
            }
            Named(_, expr) => {
                // Named expression don't do shit here.
                self.test_expression(expr)
            }
            Seq(first, second) => {
                let start = self.parser.mark();

                // Match the first, then the second.
                // If the second doesn't match, we need to manually rollback.
                match self.test_expression(first) {
                    true => match self.test_expression(second) {
                        true => true,
                        false => {
                            self.parser.reset_to(start);
                            false
                        }
                    },
                    false => false,
                }
            }
            Choice(left, right) => {
                // Rely on short circuiting to test the second part only if the first one doesn't
                // match.
                // No manual rollback needed here.
                self.test_expression(left) || self.test_expression(right)
            }
            Repetition { expr, min, max } => {
                let start = self.parser.mark();

                let max = max.unwrap_or(u32::MAX);
                let mut matches = 0;

                // Greedily match as much as possible.
                while matches < max {
                    match self.test_expression(expr) {
                        true => matches += 1,
                        false => break,
                    }
                }

                if *min <= matches && matches <= max {
                    true
                } else {
                    self.parser.reset_to(start);
                    false
                }
            }
            Predicate { expr, positive } => {
                let start = self.parser.mark();

                let matches = self.test_expression(expr);

                // Always rollback when doing a predicate.
                self.parser.reset_to(start);

                matches == *positive
            }
            Anything => self.parser.anything().is_some(),
            Epsilon => true,
        }
    }
}
