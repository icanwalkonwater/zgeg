use tree_sitter::Parser;
use zgegc::ast::AstManager;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();
    let source = std::fs::read_to_string(input_file).unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_zgeg::LANGUAGE.into())
        .unwrap();

    let mut ast_manager = AstManager::default();
    let ast = zgegc::ast::parse(&mut ast_manager, &mut parser, &source);

    dbg!(ast_manager.get(ast));
}
