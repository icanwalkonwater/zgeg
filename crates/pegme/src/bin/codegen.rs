fn main() {
    let meta_grammar = pegme::meta::make_meta_grammar();
    let code = pegme::codegen::parser_for_grammar(&meta_grammar, "MetaPegme".into());

    println!("{code}");
}
