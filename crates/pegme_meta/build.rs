use std::{fs, path::Path};

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parser.rs");

    let meta_grammar = pegme_meta_grammar::make_meta_grammar();
    let code = pegme_codegen::parser_for_grammar(
        &meta_grammar,
        "MetaPegme".into(),
        "File",
        Default::default(),
    );

    fs::write(dest_path, code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}
