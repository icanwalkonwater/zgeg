use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct AppArgs {
    src: PathBuf,
}

fn main() {
    let args = AppArgs::parse();
    dbg!(args);
}
