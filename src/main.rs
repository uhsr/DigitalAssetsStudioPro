// src/main.rs
/*
 * Main executable for DigitalAssetsStudioPro
 */

use clap::Parser;
use digitalassetsstudiopro::{Result, run};

#[derive(Parser)]
#[command(version, about = "DigitalAssetsStudioPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
