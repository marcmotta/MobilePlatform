// src/main.rs
/*
 * Main executable for MobilePlatform
 */

use clap::Parser;
use mobileplatform::{Result, run};

#[derive(Parser)]
#[command(version, about = "MobilePlatform - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
