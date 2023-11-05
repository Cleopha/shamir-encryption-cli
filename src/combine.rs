use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use crate::shamir;

/// Combines data from multiple "shard" files into a single secret file.
///
/// # Arguments
///
/// * `shard_paths` - A slice of `String` containing the paths to the shards to be combined.
/// * `output_path` - The path where the combined result file will be saved.
///
/// # Returns
///
/// This function returns an `io::Result<()>`. On success, it returns `Ok(())`.
/// On failure, it returns an `io::Error` that can occur during the reading of shard files
/// or writing of the secret file.
///
/// # Examples
///
/// ```
/// let shard_paths = vec!["./shard1.txt".to_string(), "./shard2.txt".to_string()];
/// let output_path = Path::new("./secret_combined.txt");
/// let result = combine_files(&shard_paths, &output_path);
///
/// assert!(result.is_ok());
/// ```
fn combine_files(shard_paths: &[String], output_path: &Path) -> io::Result<()> {
    let mut parts = Vec::new();

    for shard_path in shard_paths {
        let mut file = File::open(shard_path)?;
        let mut shard_data = Vec::new();
        file.read_to_end(&mut shard_data)?;
        parts.push(shard_data);
    }

    let secret = shamir::combine(parts);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&secret)?;

    Ok(())
}


/// Orchestrates the combination of shard files found within a specified directory
/// and writes the result to the given file path.
///
/// # Arguments
///
/// * `shards_dir` - A `PathBuf` pointing to the directory containing the shards.
/// * `recovered_secret_path` - A `PathBuf` specifying the path where the recovered secret will be written.
///
/// # Returns
///
/// This function returns an `io::Result<()>`. On success, it returns `Ok(())`.
/// On failure, it returns an `io::Error`.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let shards_dir = PathBuf::from("./shards");
/// let recovered_secret_path = PathBuf::from("./recovered_secret.txt");
/// let result = combine_secret(&shards_dir, &recovered_secret_path);
///
/// assert!(result.is_ok());
/// ```
pub fn combine_secret(shards_dir: &PathBuf, recovered_secret_path: &PathBuf) -> io::Result<()> {
    let shard_paths: Vec<String> = std::fs::read_dir(shards_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .collect();

    combine_files(&shard_paths, recovered_secret_path.as_path())
}
