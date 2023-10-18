use clap::*;

mod block_breaker;
mod tilesets;

#[derive(Debug, Parser)]
#[clap(name = "bevy_demos", version)]
struct App {
    #[clap(subcommand)]
    demo: Demo,
}

#[derive(Debug, Subcommand)]
enum Demo {
    /// Play a block breaker game.
    BlockBreaker,

    /// Interact with tilesets at runtime.
    Tilesets,
}

fn main() {
    match App::parse().demo {
        Demo::BlockBreaker => block_breaker::run(),
        Demo::Tilesets => tilesets::run(),
    }
}
