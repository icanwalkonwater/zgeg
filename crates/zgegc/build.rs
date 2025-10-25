use std::{fs, path::Path};

use pegme_codegen::CodegenOptions;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parser.rs");

    let grammar = pegme::meta::parse(fs::read_to_string("./zgeg.peg").unwrap()).unwrap();

    let code = pegme_codegen::parser_for_grammar(
        &grammar,
        "Zgeg".into(),
        "File",
        CodegenOptions {
            pegme_core_import: "pegme".into(),
        },
    );

    fs::write(dest_path, code).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=zgeg.peg");
}
