mod cli;
mod config;
mod display;
mod logo;
mod modules;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = cli::Args::parse();

    // Load configuration (handles priority: CLI > User Config > System Config > Defaults)
    let config = config::Config::load(&args)?;

    // Initialize system information collector
    let system_info = modules::SystemInfo::new(&config)?;

    // Render output
    display::render(&system_info, &config)?;

    Ok(())
}
