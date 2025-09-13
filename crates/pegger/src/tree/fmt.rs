use std::fmt::{self, Display, Formatter};

use super::{ParseNode, ParseTree};

impl Display for ParseTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_node(&self.root, f, 0)
    }
}

fn write_node(node: &ParseNode, f: &mut Formatter<'_>, ident: usize) -> fmt::Result {
    writeln!(
        f,
        "{:ident$}{} {}..{}",
        "",
        node.kind,
        node.span.0,
        node.span.1,
        ident = ident * 2
    )?;
    for child in &node.children {
        write_node(child, f, ident + 1)?;
    }
    Ok(())
}
