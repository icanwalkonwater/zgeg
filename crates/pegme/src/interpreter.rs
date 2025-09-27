use std::sync::Arc;

use crate::{
    cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder},
    grammar::{PegExpression, PegGrammar, PegRuleName, PegTerminal},
    packrat::PackratParser,
};

mod v2;

pub fn parse_with_grammar(
    g: &PegGrammar,
    rule: &'static str,
    input: impl Into<String>,
) -> Option<Arc<ConcreteSyntaxTree<&'static str>>> {
    let mut parser = PackratParser::new(input);
    let mut tree = ConcreteSyntaxTreeBuilder::default();

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
    parser: &'p mut PackratParser<PegRuleName, Arc<ConcreteSyntaxTree<&'static str>>>,
    tree: &'t mut ConcreteSyntaxTreeBuilder<&'static str>,
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
        let start = self.parser.mark();
        let tree_checkpoint = self.tree.checkpoint();

        let matches = match expr {
            PegExpression::Terminal(PegTerminal::Exact(lit)) => {
                let res = self.parser.expect(lit);
                if res {
                    self.tree.push_tokens(lit);
                }
                res
            }
            PegExpression::Terminal(PegTerminal::CharacterRanges(ranges)) => {
                if let Some(c) = self
                    .parser
                    .eat_if(|c| ranges.iter().any(|&(from, to)| from <= c && c <= to))
                {
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Terminal(PegTerminal::PredefinedAscii) => {
                if let Some(c) = self.parser.eat_if(|c| c.is_ascii()) {
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Terminal(PegTerminal::PredefinedUtf8Whitespace) => {
                if let Some(c) = self.parser.eat_if(char::is_whitespace) {
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Terminal(PegTerminal::PredefinedUtf8XidStart) => {
                if let Some(c) = self.parser.eat_if(unicode_id_start::is_id_start) {
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Terminal(PegTerminal::PredefinedUtf8XidContinue) => {
                if let Some(c) = self.parser.eat_if(unicode_id_start::is_id_continue) {
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Rule(rule) => self.eval_nonterminal(*rule),
            PegExpression::Named(_name, expr) => self.eval_expression(expr),
            PegExpression::Seq(first, second) => {
                self.eval_expression(first) && self.eval_expression(second)
            }
            PegExpression::Choice(first, second) => {
                if !self.eval_expression(first) {
                    self.parser.reset_to(start);
                    self.tree.restore_checkpoint(tree_checkpoint.clone());

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
                    let tree_checkpoint = self.tree.checkpoint();

                    if !self.eval_expression(expr) {
                        // Backtrack the failed match.
                        self.parser.reset_to(mark);
                        self.tree.restore_checkpoint(tree_checkpoint);
                        break;
                    }
                    matches += 1;
                }

                *min <= matches && matches <= max
            }
            PegExpression::Predicate { expr, positive } => {
                let mark = self.parser.mark();
                let tree_checkpoint = self.tree.checkpoint();

                let matches = self.eval_expression(expr);

                // Always backtrack a predicate.
                self.parser.reset_to(mark);
                self.tree.restore_checkpoint(tree_checkpoint);

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
                    self.tree.push_token(c);
                    true
                } else {
                    false
                }
            }
            PegExpression::Epsilon => true,
        };

        if !matches {
            self.parser.reset_to(start);
            self.tree.restore_checkpoint(tree_checkpoint);
        }
        matches
    }
}
