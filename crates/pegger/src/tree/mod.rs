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
pub struct ExactParseToken {
    text: Arc<str>,
}

impl ExactParseToken {
    pub fn new(text: Arc<str>) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExactParseNodeOrToken<K> {
    Node(Arc<ExactParseNode<K>>),
    Token(Arc<ExactParseToken>),
}

impl<K: Clone> ExactParseNodeOrToken<K> {
    pub fn len(&self) -> ExactParseNodeSize {
        match self {
            Self::Node(n) => n.len,
            Self::Token(t) => t.text.len(),
        }
    }
}

pub type ExactParseNodeSize = usize;
