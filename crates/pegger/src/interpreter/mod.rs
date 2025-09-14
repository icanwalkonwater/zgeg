use std::sync::Arc;

use crate::{
    grammar::{PegCharacterClass, PegExpression, PegGrammar, PegRuleName},
    packrat::PackratParser,
    tree::{ExactParseNode, ExactParseTree, ExactParseTreeBuilder},
};

pub fn parse_with_grammar(
    g: &PegGrammar,
    rule: &'static str,
    input: impl Into<String>,
) -> Option<ExactParseTree<&'static str>> {
    let mut parser = PackratParser::new(input);
    let mut tree = ExactParseTreeBuilder::default();

    let mut state = PegInterpreterState {
        grammar: g,
        parser: &mut parser,
        tree: &mut tree,
    };
    let matches = state.eval_nonterminal(PegRuleName(rule));

    match matches {
        true => Some(tree.build()),
        false => {
            println!("Partial tree: {tree:#?}");
            None
        }
    }
}

struct PegInterpreterState<'g, 'p, 't> {
    grammar: &'g PegGrammar,
    parser: &'p mut PackratParser<PegRuleName, Arc<ExactParseNode<&'static str>>>,
    tree: &'t mut ExactParseTreeBuilder<&'static str>,
}

impl PegInterpreterState<'_, '_, '_> {
    fn eval_nonterminal(&mut self, rule: PegRuleName) -> bool {
        let start = self.parser.mark();

        if let Some(memo) = self.parser.memo(rule, start) {
            // There is a result cached.
            match memo {
                Some((end, node)) => {
                    self.parser.reset_to(end);
                    self.tree.insert_node(node);
                    return true;
                }
                None => {
                    self.parser.reset_to(start);
                    return false;
                }
            }
        }

        let expr = self.grammar.rule(rule).expr();

        self.tree.start_node(rule.0);
        if self.eval_expression(expr) {
            // Success, finish the subtree and memoize it.
            let end = self.parser.mark();
            let node = self.tree.finish_node();
            self.parser.memoize_match(rule, start, end, node);

            true
        } else {
            // Failed, trash the subtree and reset the parser.
            self.tree.trash_node();
            self.parser.reset_to(start);
            self.parser.memoize_miss(rule, start);

            false
        }
    }

    fn eval_expression(&mut self, expr: &PegExpression) -> bool {
        let mark = self.parser.mark();
        let matches = match expr {
            PegExpression::LiteralExact(lit) => {
                if self.parser.expect(lit) {
                    self.tree.push_tokens(lit);
                    true
                } else {
                    false
                }
            }
            PegExpression::LiteralRange { from, to } => {
                if let Some(c) = self.parser.eat().filter(|c| from <= c && c <= to) {
                    self.tree.push_tokens(&c.to_string());
                    true
                } else {
                    false
                }
            }
            PegExpression::LiteralClass(class) => {
                if let Some(c) = self.parser.eat() {
                    let res = match class {
                        PegCharacterClass::UserDefined(ranges) => {
                            ranges.iter().any(|[from, to]| *from <= c && c <= *to)
                        }
                        PegCharacterClass::Ascii => c.is_ascii(),
                        PegCharacterClass::Utf8Whitespace => c.is_whitespace(),
                        PegCharacterClass::Utf8XidStart => unicode_id_start::is_id_start(c),
                        PegCharacterClass::Utf8XidContinue => unicode_id_start::is_id_continue(c),
                    };
                    if res {
                        self.tree.push_tokens(&c.to_string());
                    }
                    res
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
                self.tree.pause_parenting();

                let matches = self.eval_expression(expr);

                // Very ugly way of not counting the lookahead generated nodes.
                self.tree.resume_parenting();
                self.parser.reset_to(mark);

                // scary shit
                //   - bitsneak (probably)
                if *positive {
                    matches
                } else {
                    !matches
                }
            }
            PegExpression::Anything => {
                if let Some(c) = self.parser.eat() {
                    self.tree.push_tokens(&c.to_string());
                    true
                } else {
                    false
                }
            }
            PegExpression::Nothing => true,
        };

        if !matches {
            self.parser.reset_to(mark);
        }
        matches
    }
}
