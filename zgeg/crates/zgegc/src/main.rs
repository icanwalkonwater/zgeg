use tree_sitter::Parser;

fn main() {
    let input_file = std::env::args().nth(1).unwrap();
    let source = std::fs::read_to_string(input_file).unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_zgeg::LANGUAGE.into())
        .unwrap();

    let tree = parser.parse(source, None).unwrap();

    let root = tree.root_node();

    dbg!(root);
}
