use clap::Parser;
use rusty_notes::{app::run, cli::Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}
