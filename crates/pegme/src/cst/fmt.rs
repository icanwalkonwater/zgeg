use std::fmt::{self, Display, Formatter};

use super::ConcreteSyntaxTree;

impl<K: Clone + Display> Display for ConcreteSyntaxTree<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_node(self, f, 0)
    }
}

fn write_node<K: Clone + Display>(
    node: &ConcreteSyntaxTree<K>,
    f: &mut Formatter<'_>,
    ident: usize,
) -> fmt::Result {
    match node {
        ConcreteSyntaxTree::Node {
            kind,
            len,
            children,
        } => {
            writeln!(f, "{:ident$}{kind} {len}", "", ident = ident * 2)?;
            for child in children.iter() {
                write_node(child, f, ident + 1)?;
            }
        }
        ConcreteSyntaxTree::Leaf { text } => {
            writeln!(
                f,
                "{:ident$}LITERAL {}  \"{}\"",
                "",
                text.len(),
                text.escape_default(),
                ident = ident * 2
            )?;
        }
    }
    Ok(())
}
