use clap::Parser;

use arrow::cli::*;

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Some(cmd) => handle(&cmd),
        None => {}
    }
}
