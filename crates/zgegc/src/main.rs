use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct AppArgs {
    src: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = AppArgs::parse();
    dbg!(&args);

    let input = std::fs::read_to_string(&args.src)?;

    let cst = zgegc::parser::parse(input);
    dbg!(cst);
    Ok(())
}
