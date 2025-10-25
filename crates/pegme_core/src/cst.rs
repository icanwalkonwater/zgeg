mod builder;
mod fmt;
use std::sync::Arc;

pub use builder::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcreteSyntaxTree<K> {
    Node {
        kind: K,
        len: usize,
        children: Arc<[Arc<ConcreteSyntaxTree<K>>]>,
    },
    Leaf {
        text: Arc<str>,
    },
}

impl<K> ConcreteSyntaxTree<K> {
    pub fn empty_node(kind: K) -> Self {
        Self::Node {
            kind,
            len: 0,
            children: Default::default(),
        }
    }

    pub fn node(kind: K, children: Vec<Arc<ConcreteSyntaxTree<K>>>) -> Self {
        let len = children.iter().map(|t| t.len()).sum();
        Self::Node {
            kind,
            len,
            children: children.into(),
        }
    }

    pub fn leaf(text: Arc<str>) -> Self {
        Self::Leaf { text }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Node { len, .. } => *len,
            Self::Leaf { text, .. } => text.len(),
        }
    }
}
