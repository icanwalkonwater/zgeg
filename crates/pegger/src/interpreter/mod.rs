use std::time::Duration;

use crate::{
    grammar::{PegCharacterClass, PegExpression, PegGrammar, PegRuleName},
    packrat::PackratParser,
    tree::{ParseTree, ParseTreeBuilder, ParseTreeNode},
};

pub fn parse_with_grammar(
    g: &PegGrammar,
    rule: &'static str,
    input: impl Into<String>,
) -> Option<ParseTree> {
    let mut parser = PackratParser::new(input);
    let mut tree = ParseTreeBuilder::default();

    let mut state = PegInterpreterState {
        grammar: g,
        parser: &mut parser,
        tree: &mut tree,
    };
    let matches = state.eval_nonterminal(PegRuleName(rule));

    match matches {
        true => Some(tree.into_tree()),
        false => {
            println!("Partial tree: {tree:#?}");
            None
        }
    }
}

struct PegInterpreterState<'g, 'p, 't> {
    grammar: &'g PegGrammar,
    parser: &'p mut PackratParser<PegRuleName, ParseTreeNode>,
    tree: &'t mut ParseTreeBuilder,
}

enum PegInterpreterResult {
    Match(ParseTreeNode),
    NoMatch,
}

impl PegInterpreterState<'_, '_, '_> {
    fn eval_nonterminal(&mut self, rule: PegRuleName) -> bool {
        let start = self.parser.mark();

        if let Some(memo) = self.parser.memo(rule, start) {
            // There is a result cached.
            println!("Eval {rule} at {} (memo)", start.offset());
            match memo {
                Some((end, node)) => {
                    self.parser.reset_to(end);
                    self.tree.push_node(node);
                    return true;
                }
                None => {
                    self.parser.reset_to(start);
                    return false;
                }
            }
        }

        println!("Eval {rule} at {}", start.offset());

        self.tree.begin_node(rule.0, self.parser.position());

        let mut matches = false;
        for choice in self.grammar.rule(rule).choices() {
            if self.eval_expression(choice) {
                // Stop at the first match
                matches = true;
                break;
            }
            self.parser.reset_to(start);
        }

        if matches {
            let end = self.parser.mark();
            let node = self.tree.end_node(end.offset());
            self.parser.memoize_match(rule, start, end, node);
            true
        } else {
            self.tree.abandon_node();
            self.parser.memoize_miss(rule, start);
            self.parser.reset_to(start);
            false
        }
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
            PegExpression::Rule(rule) => self.eval_nonterminal(*rule),
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
                let tree_mark = self.tree.current_node_children_count();

                let matches = self.eval_expression(expr);

                self.parser.reset_to(mark);
                // Very ugly way of not counting the lookahead generated nodes.
                self.tree.cut_current_node_children(tree_mark);

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
