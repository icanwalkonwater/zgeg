use tracing::level_filters::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()
                .unwrap(),
        )
        .init();

    let file = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file).unwrap();

    let grammar = pegme_meta::parse(input).unwrap();

    println!("{grammar}");
}
