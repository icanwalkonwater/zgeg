use std::fmt::{self, Debug, Display, Formatter};

use super::{ExactParseNodeOrToken, ExactParseTree};

impl<K: Clone + Display> Display for ExactParseTree<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_node(&ExactParseNodeOrToken::Node(self.root.clone()), f, 0)
    }
}

fn write_node<K: Clone + Display>(
    node: &ExactParseNodeOrToken<K>,
    f: &mut Formatter<'_>,
    ident: usize,
) -> fmt::Result {
    writeln!(
        f,
        "{:ident$}{} {}",
        "",
        node.kind(),
        node.len(),
        ident = ident * 2
    )?;
    if let ExactParseNodeOrToken::Node(node) = node {
        for child in &node.children {
            write_node(child, f, ident + 1)?;
        }
    }
    Ok(())
}
