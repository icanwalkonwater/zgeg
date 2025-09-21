use pegme::interpreter::parse_with_grammar;
use zgegc::parser::make_zgeg_grammar;

fn main() {
    let file = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    dbg!(&file);

    let g = make_zgeg_grammar();
    if let Some(tree) = parse_with_grammar(&g, "File", file) {
        println!("{tree}");
    } else {
        println!("Syntax error");
    }
}
