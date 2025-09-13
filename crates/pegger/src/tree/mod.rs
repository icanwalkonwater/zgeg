mod builder;
mod fmt;
use std::{ops::Deref, sync::Arc};

pub use builder::*;

#[derive(Debug, PartialEq)]
pub struct ExactParseTree<K> {
    root: Arc<ExactParseNode<K>>,
}

impl<K> ExactParseTree<K> {
    pub fn from_root(root: Arc<ExactParseNode<K>>) -> Self {
        Self { root }
    }
}

impl<K> Deref for ExactParseTree<K> {
    type Target = ExactParseNode<K>;
    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ExactParseNode<K> {
    kind: K,
    len: ExactParseNodeSize,
    children: Vec<ExactParseNodeOrToken<K>>,
}

impl<K: Clone> ExactParseNode<K> {
    pub fn new(kind: K, len: ExactParseNodeSize, children: Vec<ExactParseNodeOrToken<K>>) -> Self {
        Self {
            kind,
            len,
            children,
        }
    }

    pub fn kind(&self) -> K {
        self.kind.clone()
    }

    pub fn len(&self) -> ExactParseNodeSize {
        self.len
    }

    pub fn children(&self) -> &[ExactParseNodeOrToken<K>] {
        &self.children
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ExactParseToken<K> {
    kind: K,
    text: Arc<str>,
}

impl<K: Clone> ExactParseToken<K> {
    pub fn new(kind: K, text: Arc<str>) -> Self {
        Self { kind, text }
    }

    pub fn kind(&self) -> K {
        self.kind.clone()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExactParseNodeOrToken<K> {
    Node(Arc<ExactParseNode<K>>),
    Token(Arc<ExactParseToken<K>>),
}

impl<K: Clone> ExactParseNodeOrToken<K> {
    pub fn kind(&self) -> K {
        match self {
            Self::Node(n) => n.kind.clone(),
            Self::Token(t) => t.kind.clone(),
        }
    }

    pub fn len(&self) -> ExactParseNodeSize {
        match self {
            Self::Node(n) => n.len,
            Self::Token(t) => t.text.len(),
        }
    }
}

pub type ExactParseNodeSize = usize;
