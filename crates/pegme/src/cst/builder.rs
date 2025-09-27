use std::{borrow::Borrow, collections::HashSet, hash::Hash};

use super::*;

#[derive(Debug, Default)]
pub struct ConcreteSyntaxTreeBuilder<K> {
    interner_trees: Interner<ConcreteSyntaxTree<K>>,
    interner_text: Interner<str>,
    stack: Vec<BuilderState<K>>,
    tree: Option<Arc<ConcreteSyntaxTree<K>>>,
}

#[derive(Debug, Clone)]
struct BuilderState<K> {
    kind: K,
    children: Vec<Arc<ConcreteSyntaxTree<K>>>,
    pending_tokens: String,
}

impl<K: Clone + Eq + Hash> ConcreteSyntaxTreeBuilder<K> {
    pub fn push_tokens(&mut self, text: &str) {
        let state = self.current().unwrap();
        state.pending_tokens.push_str(text);
    }

    pub fn push_token(&mut self, c: char) {
        let state = self.current().unwrap();
        state.pending_tokens.push(c);
    }

    fn flush_pending_tokens(&mut self) {
        if let Some(state) = self.stack.last_mut() {
            if state.pending_tokens.is_empty() {
                return;
            }

            let text = self.interner_text.intern(state.pending_tokens.as_str());
            let token = self.interner_trees.intern(ConcreteSyntaxTree::leaf(text));

            state.children.push(token);
            state.pending_tokens.clear();
        }
    }

    pub fn start_node(&mut self, kind: K) {
        self.flush_pending_tokens();

        self.stack.push(BuilderState {
            kind,
            children: Default::default(),
            pending_tokens: String::with_capacity(10),
        });
    }

    pub fn finish_node(&mut self) -> Arc<ConcreteSyntaxTree<K>> {
        self.flush_pending_tokens();

        let BuilderState { kind, children, .. } = self.stack.pop().unwrap();
        let node = ConcreteSyntaxTree::node(kind, children);
        let node = self.interner_trees.intern(node);

        self.insert_node(node.clone());

        node
    }

    pub fn insert_node(&mut self, node: Arc<ConcreteSyntaxTree<K>>) {
        self.flush_pending_tokens();

        if let Some(state) = self.current() {
            state.children.push(node.clone());
        } else {
            self.tree = Some(node.clone());
        }
    }

    pub fn trash_node(&mut self) {
        self.stack.pop().unwrap();
    }

    pub fn build(self) -> Arc<ConcreteSyntaxTree<K>> {
        // TODO: assert no cycles
        self.tree.unwrap()
    }

    pub fn checkpoint(&mut self) -> Checkpoint<K> {
        let state = self.current().unwrap();
        Checkpoint {
            state: state.clone(),
        }
    }

    pub fn restore_checkpoint(&mut self, checkpoint: Checkpoint<K>) {
        let state = self.current().unwrap();
        *state = checkpoint.state;
    }

    fn current(&mut self) -> Option<&mut BuilderState<K>> {
        self.stack.last_mut()
    }
}

#[derive(Clone)]
pub struct Checkpoint<K> {
    state: BuilderState<K>,
}

#[derive(Debug)]
struct Interner<T>
where
    T: ?Sized,
{
    cache: HashSet<Arc<T>>,
}

impl<T: ?Sized> Default for Interner<T> {
    fn default() -> Self {
        Self {
            cache: HashSet::default(),
        }
    }
}

impl<T: Eq + Hash + ?Sized> Interner<T> {
    pub fn intern(&mut self, t: impl Into<Arc<T>> + Borrow<T>) -> Arc<T> {
        match self.cache.get(t.borrow()) {
            Some(t) => t.clone(),
            None => {
                let arced = t.into();
                self.cache.insert(arced.clone());
                arced
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::cst::{ConcreteSyntaxTree, ConcreteSyntaxTreeBuilder};

    #[test]
    fn simple() {
        let mut builder = ConcreteSyntaxTreeBuilder::default();

        builder.start_node("hello");
        builder.start_node("hi");
        builder.push_tokens("aa");
        builder.finish_node();
        builder.start_node("ho");
        builder.push_tokens("b");
        builder.finish_node();
        builder.finish_node();

        let t = builder.build();

        pretty_assertions::assert_eq!(
            t,
            Arc::new(ConcreteSyntaxTree::node(
                "hello",
                vec![
                    Arc::new(ConcreteSyntaxTree::node(
                        "hi",
                        vec![Arc::new(ConcreteSyntaxTree::leaf(Arc::from("aa")))]
                    )),
                    Arc::new(ConcreteSyntaxTree::node(
                        "ho",
                        vec![Arc::new(ConcreteSyntaxTree::leaf(Arc::from("b")))]
                    ))
                ]
            )),
        );
    }

    #[test]
    fn token_concat() {
        let mut builder = ConcreteSyntaxTreeBuilder::default();

        builder.start_node("hello");
        builder.push_tokens("a");
        builder.push_tokens("b");
        let ck = builder.checkpoint();
        builder.push_tokens("c");
        builder.restore_checkpoint(ck);
        builder.push_tokens("de");
        builder.finish_node();

        let t = builder.build();

        pretty_assertions::assert_eq!(
            t,
            Arc::new(ConcreteSyntaxTree::node(
                "hello",
                vec![Arc::new(ConcreteSyntaxTree::leaf(Arc::from("abde")))]
            ))
        );
    }

    #[test]
    fn skip_some() {
        let mut builder = ConcreteSyntaxTreeBuilder::default();

        builder.start_node("hello");
        builder.start_node("hi");
        builder.push_tokens("aa");
        builder.finish_node();
        let ck = builder.checkpoint();
        builder.push_tokens("owo");
        builder.start_node("owo");
        builder.push_tokens("uwu");
        builder.finish_node();
        builder.restore_checkpoint(ck);
        builder.push_tokens("b");
        builder.finish_node();

        let t = builder.build();

        pretty_assertions::assert_eq!(
            t,
            Arc::new(ConcreteSyntaxTree::node(
                "hello",
                vec![
                    Arc::new(ConcreteSyntaxTree::node(
                        "hi",
                        vec![Arc::new(ConcreteSyntaxTree::leaf(Arc::from("aa")))]
                    )),
                    Arc::new(ConcreteSyntaxTree::leaf(Arc::from("b")))
                ]
            )),
        );
    }
}
