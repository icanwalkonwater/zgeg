use std::io::Read;

use zgegc::parser::make_zgeg_grammar;

fn main() {
    let input_path = std::env::args().nth(1).unwrap();
    let file = if &input_path == "-" {
        let mut b = String::new();
        std::io::stdin().lock().read_to_string(&mut b).unwrap();
        b
    } else {
        std::fs::read_to_string(input_path).unwrap()
    };

    dbg!(&file, file.len());

    let g = make_zgeg_grammar();
    if let Some(tree) = pegme::interpreter::parse_with_grammar(&g, "File", file) {
        println!("{tree}");
    } else {
        println!("Syntax error");
    }
}
