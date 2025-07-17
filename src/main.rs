// src/main.rs
/*
 * Main executable for BlockchainInteroperabilityHub
 */

use clap::Parser;
use blockchaininteroperabilityhub::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainInteroperabilityHub - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
