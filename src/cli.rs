use clap::{Parser, Subcommand};

/// Rust-based command-line application that implements Shamir's Secret Sharing algorithm
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Commands supported by the CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Shard a secret into shards
    Shard {
        /// Path to the secret file
        #[clap(parse(from_os_str))]
        secret_path: std::path::PathBuf,

        /// Path to store the shards
        #[clap(parse(from_os_str))]
        shards_path: std::path::PathBuf,

        /// Number of parts to split the secret into
        #[clap(short, long, default_value_t = 5)]
        parts: usize,

        /// Threshold number of parts required to recombine the secret
        #[clap(short, long, default_value_t = 3)]
        threshold: usize,
    },
    /// Combine shards into a secret
    Combine {
        /// Directory path containing the shards
        #[clap(parse(from_os_str))]
        shards_dir: std::path::PathBuf,

        /// Path to store the recovered secret
        #[clap(parse(from_os_str))]
        recovered_secret_path: std::path::PathBuf,
    },
}
