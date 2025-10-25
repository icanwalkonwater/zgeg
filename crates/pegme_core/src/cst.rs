mod builder;
mod fmt;
use std::sync::Arc;

pub use builder::*;
use itertools::Either;

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

impl<K> ConcreteSyntaxTree<K>
where
    K: Clone + PartialEq,
{
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

    pub fn kind(&self) -> Option<K> {
        match self {
            Self::Node { kind, .. } => Some(kind.clone()),
            Self::Leaf { .. } => None,
        }
    }

    pub fn is(&self, other: K) -> bool {
        match self {
            Self::Node { kind, .. } => other == *kind,
            Self::Leaf { .. } => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Node { len, .. } => *len,
            Self::Leaf { text, .. } => text.len(),
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::Node { .. } => "",
            Self::Leaf { text } => text,
        }
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &Arc<ConcreteSyntaxTree<K>>> {
        match self {
            Self::Node { children, .. } => Either::Left(children.iter().map(|t| t)),
            Self::Leaf { .. } => Either::Right(std::iter::empty()),
        }
    }

    pub fn find_child_by_kind(&self, kind: K) -> Option<&Arc<ConcreteSyntaxTree<K>>> {
        self.iter_children().find(|t| t.is(kind.clone()))
    }

    pub fn find_children_by_kind(
        &self,
        kind: K,
    ) -> impl Iterator<Item = &Arc<ConcreteSyntaxTree<K>>> {
        self.iter_children().filter(move |t| t.is(kind.clone()))
    }
}
