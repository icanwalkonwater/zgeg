use std::fmt::{self, Display, Formatter};

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
    match node {
        ExactParseNodeOrToken::Node(node) => {
            writeln!(
                f,
                "{:ident$}{} {}",
                "",
                node.kind(),
                node.len(),
                ident = ident * 2
            )?;
            for child in node.children() {
                write_node(child, f, ident + 1)?;
            }
        }
        ExactParseNodeOrToken::Token(token) => {
            writeln!(
                f,
                "{:ident$}LITERAL {}  \"{}\"",
                "",
                node.len(),
                token.text.escape_default(),
                ident = ident * 2
            )?;
        }
    }
    Ok(())
}
