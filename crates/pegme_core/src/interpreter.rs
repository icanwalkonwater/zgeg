use std::sync::Arc;

use itertools::Itertools;

use crate::{
    cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder},
    grammar::{Grammar, PegExpression, PegTerminal},
    packrat::{PackratMark, PackratParser},
};

pub fn parse_with_grammar(
    g: &Grammar,
    root: &'static str,
    input: String,
) -> Option<Arc<ConcreteSyntaxTree<Arc<str>>>> {
    let mut state = InterpreterState {
        grammar: g,
        parser: PackratParser::new(input),
        tree: ConcreteSyntaxTreeBuilder::default(),
    };

    // This also initializes the packrat memos.
    let matches = state.test_expression(
        &PegExpression::NonTerminal {
            rule_name: root.into(),
        },
        false,
    );

    if !matches {
        println!("No match");
        return None;
    }

    // Build the CST.
    state.parser.reset();
    state.parse_rule(root.into());

    Some(state.tree.build())
}

struct InterpreterState<'g> {
    grammar: &'g Grammar,
    parser: PackratParser<String>,
    tree: ConcreteSyntaxTreeBuilder<Arc<str>>,
}

#[derive(Debug)]
struct ScavengeReport {
    named_nodes: Vec<(String, PackratMark, PackratMark)>,
}

impl InterpreterState<'_> {
    /// This should only be called on rules that we know match.
    ///
    /// Builds the concrete syntax tree for this rule, called recursively.
    fn parse_rule(&mut self, name: String) {
        let start = self.parser.mark();
        let end = self.parser.memo(name.clone(), start).unwrap().unwrap();

        let report = self.scavenge_rule(name.clone());

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

        let node_ticket = self.tree.start_node(name.into());

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

        self.tree.finish_node(node_ticket);
    }

    /// This should only be called on rules that we know match.
    ///
    /// Scaffolding for `scavenge_expression`.
    fn scavenge_rule(&mut self, rule: String) -> ScavengeReport {
        assert!(
            matches!(
                self.parser.memo(rule.clone(), self.parser.mark()),
                Some(Some(_))
            ),
            "Tried to scavenge a rule that doesn't match: {rule:?} at {:?}",
            self.parser.mark()
        );

        let mut report = ScavengeReport {
            named_nodes: Default::default(),
        };

        let rule = self.grammar.find_rule(&rule).unwrap();
        self.scavenge_expression(rule.match_expression(), &mut report);

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
                self.test_expression(expr, true),
                "Tried to scavenge an expression that doesn't match: `{expr}` at {start:?}"
            );
            self.parser.reset_to(start);
        }

        use PegExpression::*;
        match expr {
            // Just advance terminals, there is nothing to scavenge there.
            expr @ Terminal(_) => {
                // Use the test to advance the parser.
                let res = self.test_expression(expr, true);
                assert!(res);
            }
            // This is the main thing we are looking for.
            // TODO: handle named expressions.
            NonTerminal { rule_name } | NamedNonTerminal { rule_name, .. } => {
                let start = self.parser.mark();

                // It always matches.
                let end = self.parser.memo(rule_name.clone(), start).unwrap().unwrap();
                report.named_nodes.push((rule_name.clone(), start, end));

                self.parser.reset_to(end);
            }
            Seq { left, right } => {
                self.scavenge_expression(left, report);
                self.scavenge_expression(right, report);
            }
            Choice { left, right } => {
                // We don't know if the first half matches so we need to do a bit of gymnastic to
                // figure it out.

                let start = self.parser.mark();
                if self.test_expression(left, true) {
                    // It's the first choice.
                    self.parser.reset_to(start);

                    self.scavenge_expression(left, report);
                } else {
                    // It's the second choice.
                    // No rollback since the test didn't pass and didn't consume.
                    self.scavenge_expression(right, report);
                }
            }
            Repetition { expr } => {
                // Match as much as possible.

                loop {
                    let start = self.parser.mark();
                    if self.test_expression(expr, true) {
                        self.parser.reset_to(start);
                        self.scavenge_expression(expr, report);
                    } else {
                        break;
                    }
                }
            }
            Predicate { .. } => {
                // This is just a noop, we know it matches and there is nothing to scavenge.
            }
        }
    }

    /// Implements the PEG operators, called recursively.
    fn test_expression(&mut self, expr: &PegExpression, memo_only: bool) -> bool {
        use PegExpression::*;
        match expr {
            Terminal(PegTerminal::Epsilon) => true,
            Terminal(PegTerminal::Any) => self.parser.anything().is_some(),
            Terminal(PegTerminal::Literal(lit)) => self.parser.expect(lit),
            Terminal(PegTerminal::Ranges(ranges)) => self
                .parser
                .eat(|c| ranges.iter().any(|range| range.contains(&c)))
                .is_some(),
            NonTerminal { rule_name } | NamedNonTerminal { rule_name, .. } => {
                // NOTE: Named expression don't do shit here.

                let start = self.parser.mark();

                // Look up the rule's memo and only test it if it doesn't pass.
                match self.parser.memo(rule_name.clone(), start) {
                    Some(Some(end)) => {
                        self.parser.reset_to(end);
                        true
                    }
                    Some(None) => false,
                    None if memo_only => {
                        panic!(
                            "Trying to match a rule that isn't memoized: rule_name at {start:?}"
                        );
                    }
                    None => {
                        let rule = self.grammar.find_rule(rule_name).unwrap();
                        let matches = self.test_expression(rule.match_expression(), memo_only);

                        if matches {
                            let end = self.parser.mark();
                            self.parser.memoize_match(rule_name.clone(), start, end);
                            true
                        } else {
                            self.parser.memoize_miss(rule_name.clone(), start);
                            false
                        }
                    }
                }
            }
            Seq { left, right } => {
                let start = self.parser.mark();

                // Match the first, then the second.
                // If the second doesn't match, we need to manually rollback.
                match self.test_expression(left, memo_only) {
                    true => match self.test_expression(right, memo_only) {
                        true => true,
                        false => {
                            self.parser.reset_to(start);
                            false
                        }
                    },
                    false => false,
                }
            }
            Choice { left, right } => {
                // Rely on short circuiting to test the second part only if the first one doesn't
                // match.
                // No manual rollback needed here.
                self.test_expression(left, memo_only) || self.test_expression(right, memo_only)
            }
            Repetition { expr } => {
                // Greedily match as much as possible.
                while self.test_expression(expr, memo_only) {}
                true
            }
            Predicate { expr, positive } => {
                let start = self.parser.mark();

                let matches = self.test_expression(expr, memo_only);

                // Always rollback when doing a predicate.
                self.parser.reset_to(start);

                matches == *positive
            }
        }
    }
}
