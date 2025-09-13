#[derive(Debug, PartialEq)]
pub struct ParseTree {
    root: ParseTreeNode,
}

impl ParseTree {
    pub fn from_root(root: ParseTreeNode) -> Self {
        Self { root }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseTreeNode {
    kind: &'static str,
    span: Span,
    children: Vec<ParseTreeNode>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span(pub usize, pub usize);

#[derive(Debug, Default)]
pub struct ParseTreeBuilder {
    node_stack: Vec<(NodeBuilder, Vec<ParseTreeNode>)>,
    tree: Option<ParseTree>,
}

#[derive(Debug)]
struct NodeBuilder {
    kind: &'static str,
    start: usize,
}

impl ParseTreeBuilder {
    pub fn push_node(&mut self, node: ParseTreeNode) {
        assert!(self.tree.is_none(), "A parse tree has already been built");
        if !self.node_stack.is_empty() {
            let (_, children) = self.node_stack.last_mut().unwrap();
            children.push(node);
        } else {
            self.tree = Some(ParseTree { root: node });
        }
    }

    pub fn begin_node(&mut self, kind: &'static str, offset: usize) {
        self.node_stack.push((
            NodeBuilder {
                kind,
                start: offset,
            },
            Default::default(),
        ));
    }

    pub fn current_node_children_count(&self) -> usize {
        self.node_stack.last().map(|(_, c)| c.len()).unwrap_or(0)
    }

    pub fn cut_current_node_children(&mut self, max: usize) {
        if let Some((_, c)) = self.node_stack.last_mut() {
            while c.len() > max {
                c.pop();
            }
        }
    }

    pub fn abandon_node(&mut self) {
        self.node_stack.pop();
    }

    pub fn end_node(&mut self, offset: usize) -> ParseTreeNode {
        let (builder, children) = self.node_stack.pop().unwrap();
        let node = ParseTreeNode {
            kind: builder.kind,
            span: Span(builder.start, offset),
            children,
        };
        self.push_node(node.clone());
        node
    }

    pub fn into_tree(self) -> ParseTree {
        self.tree.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{ParseTree, ParseTreeNode, Span};

    use super::ParseTreeBuilder;

    #[test]
    fn simple() {
        let mut builder = ParseTreeBuilder::default();

        builder.begin_node("hello", 0);
        builder.begin_node("hi", 0);
        builder.end_node(2);
        builder.begin_node("ho", 2);
        builder.end_node(3);
        builder.end_node(3);

        let t = builder.into_tree();

        pretty_assertions::assert_eq!(
            t,
            ParseTree {
                root: ParseTreeNode {
                    kind: "hello",
                    span: Span(0, 3),
                    children: vec![
                        ParseTreeNode {
                            kind: "hi",
                            span: Span(0, 2),
                            children: vec![],
                        },
                        ParseTreeNode {
                            kind: "ho",
                            span: Span(2, 3),
                            children: vec![],
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn skip_some() {
        let mut builder = ParseTreeBuilder::default();

        builder.begin_node("hello", 0);
        builder.begin_node("hi", 0);
        builder.end_node(2);
        let mark = builder.current_node_children_count();
        builder.begin_node("skipped", 2);
        builder.end_node(2);
        builder.cut_current_node_children(mark);
        builder.begin_node("ho", 2);
        builder.end_node(3);
        builder.end_node(3);

        let t = builder.into_tree();

        pretty_assertions::assert_eq!(
            t,
            ParseTree {
                root: ParseTreeNode {
                    kind: "hello",
                    span: Span(0, 3),
                    children: vec![
                        ParseTreeNode {
                            kind: "hi",
                            span: Span(0, 2),
                            children: vec![],
                        },
                        ParseTreeNode {
                            kind: "ho",
                            span: Span(2, 3),
                            children: vec![],
                        }
                    ]
                }
            }
        );
    }
}
