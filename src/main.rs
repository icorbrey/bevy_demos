use clap::*;

mod tilesets;

#[derive(Debug, Parser)]
#[clap(name = "bevy_demos", version)]
struct App {
    #[clap(subcommand)]
    demo: Demo,
}

#[derive(Debug, Subcommand)]
enum Demo {
    /// Interact with tilesets at runtime.
    Tilesets,
}

fn main() {
    match App::parse().demo {
        Demo::Tilesets => tilesets::run(),
    }
}
