mod builder;
mod fmt;
pub use builder::*;

#[derive(Debug, PartialEq)]
pub struct ParseTree {
    root: ParseNode,
}

impl ParseTree {
    pub fn from_root(root: ParseNode) -> Self {
        Self { root }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseNode {
    kind: &'static str,
    span: Span,
    children: Vec<ParseNode>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span(pub usize, pub usize);
