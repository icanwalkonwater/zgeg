use tree_sitter::{Node, Parser};

use crate::ast;

use super::{Ast, AstManager};

fn parse(ast_manager: &mut AstManager, parser: &mut Parser, source: &str) {
    let tree = parser.parse(source, None).unwrap();

    let file = tree.root_node();
    assert_eq!(file.kind(), "source_file");
    parse_file(ast_manager, file);
}

fn parse_file(ast_manager: &mut AstManager, node_file: Node) -> Ast<ast::File> {
    if node_file.child_count() == 0 {
        return ast_manager.push(ast::File { items: vec![] });
    }

    let mut children = node_file.walk();
    assert!(children.goto_first_child());

    let mut items = vec![];
    loop {
        items.push(parse_item(ast_manager, children.node()));
        if !children.goto_next_sibling() {
            return ast_manager.push(ast::File { items });
        }
    }
}

fn parse_item(ast_manager: &mut AstManager, item_node: Node) -> Ast<ast::Item> {
    todo!()
}
