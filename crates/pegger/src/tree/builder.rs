use std::{collections::HashSet, hash::Hash};

use super::*;

#[derive(Debug, Default)]
pub struct ExactParseTreeBuilder<K> {
    interner_nodes: HashSet<Arc<ExactParseNode<K>>>,
    interner_tokens: HashSet<Arc<ExactParseToken<K>>>,
    interner_text: HashSet<Arc<str>>,
    stack: Vec<BuilderState<K>>,
    tree: Option<ExactParseTree<K>>,
}

#[derive(Debug)]
struct BuilderState<K> {
    kind: K,
    children: Vec<ExactParseNodeOrToken<K>>,
    parenting: bool,
}

impl<K: Clone + Eq + Hash> ExactParseTreeBuilder<K> {
    pub fn push_token_node(&mut self, kind: K, text: &str) -> Arc<ExactParseToken<K>> {
        let token = ExactParseToken::new(kind, self.interned_text(text));
        let token = self.interned_token(token);

        if let Some(state) = self.current() {
            if state.parenting {
                state
                    .children
                    .push(ExactParseNodeOrToken::Token(token.clone()));
            }
        }

        token
    }

    pub fn start_node(&mut self, kind: K) {
        self.stack.push(BuilderState {
            kind,
            children: Default::default(),
            parenting: true,
        });
    }

    pub fn finish_node(&mut self) -> Arc<ExactParseNode<K>> {
        let BuilderState { kind, children, .. } = self.stack.pop().unwrap();
        let len = children.iter().map(|n| n.len()).sum::<ExactParseNodeSize>();
        let node = ExactParseNode::new(kind, len, children);
        let node = self.interned_node(node);

        self.insert_node(node.clone());

        node
    }

    pub fn insert_node(&mut self, node: Arc<ExactParseNode<K>>) {
        if let Some(state) = self.current() {
            if state.parenting {
                state
                    .children
                    .push(ExactParseNodeOrToken::Node(node.clone()));
            }
        } else {
            self.tree = Some(ExactParseTree::from_root(node.clone()));
        }
    }

    pub fn trash_node(&mut self) {
        self.stack.pop().unwrap();
    }

    pub fn build(self) -> ExactParseTree<K> {
        self.tree.unwrap()
    }

    pub fn pause_parenting(&mut self) {
        self.current().unwrap().parenting = false;
    }

    pub fn resume_parenting(&mut self) {
        self.current().unwrap().parenting = true;
    }

    fn current(&mut self) -> Option<&mut BuilderState<K>> {
        self.stack.last_mut()
    }

    fn interned_text(&mut self, text: &str) -> Arc<str> {
        match self.interner_text.get(text) {
            Some(t) => t.clone(),
            None => {
                let t = Arc::<str>::from(text);
                self.interner_text.insert(t.clone());
                t
            }
        }
    }

    fn interned_token(&mut self, token: ExactParseToken<K>) -> Arc<ExactParseToken<K>> {
        match self.interner_tokens.get(&token) {
            Some(t) => t.clone(),
            None => {
                let t = Arc::new(token);
                self.interner_tokens.insert(t.clone());
                t
            }
        }
    }

    fn interned_node(&mut self, node: ExactParseNode<K>) -> Arc<ExactParseNode<K>> {
        match self.interner_nodes.get(&node) {
            Some(t) => t.clone(),
            None => {
                let t = Arc::new(node);
                self.interner_nodes.insert(t.clone());
                t
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::tree::{
        ExactParseNode, ExactParseNodeOrToken, ExactParseToken, ExactParseTree,
        ExactParseTreeBuilder,
    };

    #[test]
    fn simple() {
        let mut builder = ExactParseTreeBuilder::default();

        builder.start_node("hello");
        builder.start_node("hi");
        builder.push_token_node("LITERAL", "aa");
        builder.finish_node();
        builder.start_node("ho");
        builder.push_token_node("LITERAL", "b");
        builder.finish_node();
        builder.finish_node();

        let t = builder.build();

        pretty_assertions::assert_eq!(
            t,
            ExactParseTree::from_root(Arc::new(ExactParseNode::new(
                "hello",
                3,
                vec![
                    ExactParseNodeOrToken::Node(Arc::new(ExactParseNode::new(
                        "hi",
                        2,
                        vec![ExactParseNodeOrToken::Token(Arc::new(
                            ExactParseToken::new("LITERAL", Arc::from("aa"))
                        ))]
                    ))),
                    ExactParseNodeOrToken::Node(Arc::new(ExactParseNode::new(
                        "ho",
                        1,
                        vec![ExactParseNodeOrToken::Token(Arc::new(
                            ExactParseToken::new("LITERAL", Arc::from("b"))
                        ))]
                    )))
                ]
            ))),
        );
    }

    #[test]
    fn skip_some() {
        let mut builder = ExactParseTreeBuilder::default();

        builder.start_node("hello");
        builder.start_node("hi");
        builder.push_token_node("LITERAL", "aa");
        builder.finish_node();
        builder.pause_parenting();
        builder.push_token_node("LITERAL", "owo");
        builder.start_node("owo");
        builder.push_token_node("LITERAL", "uwu");
        builder.finish_node();
        builder.resume_parenting();
        builder.push_token_node("LITERAL", "b");
        builder.finish_node();

        let t = builder.build();

        pretty_assertions::assert_eq!(
            t,
            ExactParseTree::from_root(Arc::new(ExactParseNode::new(
                "hello",
                3,
                vec![
                    ExactParseNodeOrToken::Node(Arc::new(ExactParseNode::new(
                        "hi",
                        2,
                        vec![ExactParseNodeOrToken::Token(Arc::new(
                            ExactParseToken::new("LITERAL", Arc::from("aa"))
                        ))]
                    ))),
                    ExactParseNodeOrToken::Token(Arc::new(ExactParseToken::new(
                        "LITERAL",
                        Arc::from("b")
                    )))
                ]
            ))),
        );
    }
}
