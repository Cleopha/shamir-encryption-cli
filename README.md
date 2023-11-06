# Shamir's Secret Sharing CLI Tool

A project that utilizes Shamir's Secret Sharing algorithm to manage sensitive data. This Rust-based command-line tool allows for the secure division of a secret into multiple shares, as well as the recombination of those shares to retrieve the original secret.

## :warning: Disclaimer

This tool is a small personal project and has not undergone any formal security audit. It is likely vulnerable to certain types of attacks, such as timing attacks, and therefore should not be used for critical security applications. It serves as a conceptual demonstration and a starting point for those interested in secret-sharing schemes.

## Getting Started

Follow these instructions to get the project up and running on your local machine for development and testing purposes.

### Prerequisites

- [Cargo](https://www.rust-lang.org/) (Rust's package manager)

### Installation

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/shamir-secret-sharing.git
cd shamir-secret-sharing
cargo build --release
```

The built executable will be located at `target/release/`.

## Usage

```sh
shamir-encryption 0.1.0
Main command line program

USAGE:
    shamir-encryption <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    combine    Combine shards into a secret
    help       Print this message or the help of the given subcommand(s)
    shard      Shard a secret into shards
```

### Creating Shares

To split a secret into multiple shares:

```sh
shamir-encryption shard --secret-path <path-to-secret> --shards-path <output-directory> --parts <number-of-shares> --threshold <shares-needed-to-recover>
```

### Recovering the Secret

To recover the original secret:

```sh
shamir-encryption combine --shards-dir <shards-directory> --recovered-secret-path <recovery-path>
```

## Testing

Test the functionality with:

```sh
cargo test
```
