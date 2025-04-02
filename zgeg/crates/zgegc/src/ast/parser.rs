use tree_sitter::{Node, Parser};

use crate::ast;

use super::{Ast, AstManager};

pub fn parse(ast_manager: &mut AstManager, parser: &mut Parser, source: &str) -> Ast<ast::File> {
    let tree = parser.parse(source, None).unwrap();

    let file = tree.root_node();
    assert_eq!(file.kind(), "source_file");
    parse_file(ast_manager, file)
}

fn parse_file(ast_manager: &mut AstManager, node_file: Node) -> Ast<ast::File> {
    let mut file = ast::File { items: vec![] };

    let mut children = node_file.walk();

    if !children.goto_first_child() {
        return ast_manager.push(file);
    }
    loop {
        file.items.push(parse_item(ast_manager, children.node()));
        if !children.goto_next_sibling() {
            break;
        }
    }

    ast_manager.push(file)
}

fn parse_item(ast_manager: &mut AstManager, node_item: Node) -> Ast<ast::Item> {
    let item = match node_item.grammar_name() {
        "function_item" => ast::Item::Function(parse_function_item(ast_manager, node_item)),
        _ => unreachable!(),
    };
    ast_manager.push(item)
}

fn parse_function_item(
    ast_manager: &mut AstManager,
    function_item_node: Node,
) -> Ast<ast::FunctionItem> {
    let name = parse_identifier(
        ast_manager,
        function_item_node.child_by_field_name("name").unwrap(),
    );
    let return_type = function_item_node
        .child_by_field_name("return_type")
        .map(|node| parse_type(ast_manager, node));
    let body = parse_block(
        ast_manager,
        function_item_node.child_by_field_name("body").unwrap(),
    );
    ast_manager.push(ast::FunctionItem {
        name,
        return_type,
        body,
    })
}

fn parse_block(ast_manager: &mut AstManager, node_block: Node) -> Ast<ast::Block> {
    let mut cursor = node_block.walk();
    let statements = node_block
        .children_by_field_name("statements", &mut cursor)
        .map(|stmt| parse_statement(ast_manager, stmt))
        .collect();
    ast_manager.push(ast::Block { statements })
}

fn parse_statement(ast_manager: &mut AstManager, node_statement: Node) -> Ast<ast::Statement> {
    let statement = match node_statement.grammar_name() {
        "let_statement" => ast::Statement::Let(parse_let_statement(ast_manager, node_statement)),
        "assignement_statement" => {
            ast::Statement::Assignement(parse_assignement_statement(ast_manager, node_statement))
        }
        "return_statement" => {
            ast::Statement::Return(parse_return_statement(ast_manager, node_statement))
        }
        _ => todo!(),
    };
    ast_manager.push(statement)
}

fn parse_let_statement(
    ast_manager: &mut AstManager,
    node_let_statement: Node,
) -> Ast<ast::LetStatement> {
    let name = parse_identifier(
        ast_manager,
        node_let_statement.child_by_field_name("name").unwrap(),
    );
    let type_ = node_let_statement
        .child_by_field_name("type")
        .map(|node| parse_type(ast_manager, node));
    let initializer = node_let_statement
        .child_by_field_name("initializer")
        .map(|node| parse_expression(ast_manager, node));

    ast_manager.push(ast::LetStatement {
        name,
        type_,
        initializer,
    })
}

fn parse_assignement_statement(
    ast_manager: &mut AstManager,
    node_assignement_statement: Node,
) -> Ast<ast::AssignementStatement> {
    let place = parse_identifier(
        ast_manager,
        node_assignement_statement
            .child_by_field_name("place")
            .unwrap(),
    );
    let value = parse_expression(
        ast_manager,
        node_assignement_statement
            .child_by_field_name("value")
            .unwrap(),
    );
    ast_manager.push(ast::AssignementStatement { place, value })
}

fn parse_return_statement(
    ast_manager: &mut AstManager,
    node_return_statement: Node,
) -> Ast<ast::ReturnStatement> {
    let return_value = node_return_statement
        .child_by_field_name("return_value")
        .map(|node| parse_expression(ast_manager, node));
    ast_manager.push(ast::ReturnStatement { return_value })
}

fn parse_expression(ast_manager: &mut AstManager, node_expression: Node) -> Ast<ast::Expression> {
    let expression = match node_expression.grammar_name() {
        "binary_expression" => {
            ast::Expression::Binary(parse_binary_expression(ast_manager, node_expression))
        }
        "unary_expression" => {
            ast::Expression::Unary(parse_unary_expression(ast_manager, node_expression))
        }
        "function_call" => {
            ast::Expression::FunctionCall(parse_function_call(ast_manager, node_expression))
        }
        "identifier" => ast::Expression::Identifier(parse_identifier(ast_manager, node_expression)),
        "literal" => ast::Expression::Literal(parse_literal(ast_manager, node_expression)),
        _ => unreachable!(),
    };
    ast_manager.push(expression)
}

fn parse_binary_expression(
    ast_manager: &mut AstManager,
    node_binary_expression: Node,
) -> Ast<ast::BinaryExpression> {
    let left = parse_expression(
        ast_manager,
        node_binary_expression.child_by_field_name("left").unwrap(),
    );
    let op = parse_binary_op(
        ast_manager,
        node_binary_expression.child_by_field_name("op").unwrap(),
    );
    let right = parse_expression(
        ast_manager,
        node_binary_expression.child_by_field_name("right").unwrap(),
    );
    ast_manager.push(ast::BinaryExpression { left, op, right })
}

fn parse_binary_op(ast_manager: &mut AstManager, node_binary_op: Node) -> Ast<ast::BinaryOp> {
    todo!("idk how to do this")
}

fn parse_unary_expression(
    ast_manager: &mut AstManager,
    node_unary_expression: Node,
) -> Ast<ast::UnaryExpression> {
    let op = parse_unary_op(
        ast_manager,
        node_unary_expression.child_by_field_name("op").unwrap(),
    );
    let value = parse_expression(
        ast_manager,
        node_unary_expression.child_by_field_name("value").unwrap(),
    );
    ast_manager.push(ast::UnaryExpression { op, value })
}

fn parse_unary_op(ast_manager: &mut AstManager, node_unary_op: Node) -> Ast<ast::UnaryOp> {
    todo!("idk")
}

fn parse_function_call(
    ast_manager: &mut AstManager,
    node_function_call: Node,
) -> Ast<ast::FunctionCall> {
    todo!()
}

fn parse_literal(ast_manager: &mut AstManager, node_literal: Node) -> Ast<ast::Literal> {
    todo!()
}

fn parse_type(ast_manager: &mut AstManager, node_type: Node) -> Ast<ast::Type> {
    todo!()
}

fn parse_identifier(ast_manager: &mut AstManager, node_identifier: Node) -> Ast<ast::Identifier> {
    todo!()
}
