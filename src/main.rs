use colored::*;

use clap::Parser;
use cli::{Cli, Commands};

use crate::combine::combine_secret;
use crate::sharding::shard_secret;

mod cli;
mod gf256;
mod polynomial;
mod shamir;
mod sharding;
mod combine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Shard {
            secret_path,
            shards_path,
            parts,
            threshold,
        } => {
            shard_secret(&secret_path, &shards_path, parts, threshold)?;
            println!("{}", "Sharding complete!".green());
            println!(
                "Secret at {} was split into {} parts with a threshold of {}.",
                shards_path.to_string_lossy().bright_blue(),
                parts.to_string().cyan(),
                threshold.to_string().cyan()
            );
        }
        Commands::Combine {
            shards_dir,
            recovered_secret_path,
        } => {
            combine_secret(&shards_dir, &recovered_secret_path)?;
            println!("{}", "Combine complete!".green());
            println!("Recovered secret saved to {}", recovered_secret_path.to_string_lossy().bright_blue());
        }
    }

    Ok(())
}
