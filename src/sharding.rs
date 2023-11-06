use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use crate::shamir;

/// Reads the contents of a file and shards it into multiple parts based on Shamir's Secret Sharing.
///
/// # Arguments
///
/// * `path` - A reference to the `Path` where the original file is located.
/// * `parts` - The number of shards to create.
/// * `threshold` - The minimum number of shards required to reconstruct the original file.
///
/// # Returns
///
/// An `io::Result` which is either:
/// - `Ok(Vec<String>)`: A vector of strings representing the file paths of the created shards.
/// - `Err(io::Error)`: An error that occurred during the sharding process.
///
/// # Examples
///
/// ```
/// let file_path = Path::new("path/to/myfile.txt");
/// match shard_file(file_path, 5, 3) {
///     Ok(shard_paths) => println!("Shards created: {:?}", shard_paths),
///     Err(e) => eprintln!("An error occurred: {}", e),
/// }
/// ```
fn shard_file(path: &Path, parts: usize, threshold: usize) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let shards = shamir::split(&data, parts, threshold);
    let mut shard_paths = Vec::new();

    for (index, shard) in shards.into_iter().enumerate() {
        let shard_path = format!("{}_{}", "shards", index);
        let mut shard_file = File::create(&shard_path)?;
        shard_file.write_all(&shard)?;
        shard_paths.push(shard_path);
    }

    Ok(shard_paths)
}

/// Shards a secret contained in a file and stores the shards in a specified directory.
///
/// # Arguments
///
/// * `secret_path` - A `PathBuf` pointing to the file that contains the secret.
/// * `shards_path` - A `PathBuf` specifying the directory where the shards should be stored.
/// * `parts` - The number of shards to create.
/// * `threshold` - The minimum number of shards required to reconstruct the secret.
///
/// # Returns
///
/// An `io::Result<()>` which is:
/// - `Ok(())`: On successful sharding of the secret.
/// - `Err(io::Error)`: If any error occurs during the sharding process.
///
/// # Panics
///
/// The function does not explicitly panic, but filesystem operations may panic if permissions
/// are insufficient or if unexpected conditions are met (like a full disk).
///
/// # Examples
///
/// ```
/// let secret_file = PathBuf::from("path/to/secret.txt");
/// let shards_directory = PathBuf::from("path/to/shards");
/// match shard_secret(&secret_file, &shards_directory, 5, 3) {
///     Ok(()) => println!("Secret successfully sharded."),
///     Err(e) => eprintln!("Failed to shard the secret: {}", e),
/// }
/// ```
pub fn shard_secret(
    secret_path: &Path,
    shards_path: &PathBuf,
    parts: usize,
    threshold: usize,
) -> io::Result<()> {
    // Check if the shards directory exists, if not, create it.
    if !shards_path.exists() {
        fs::create_dir_all(shards_path)?;
    }

    let shard_paths = shard_file(secret_path, parts, threshold)?;

    // Move shard files to the desired directory if necessary
    let mut new_shard_paths = Vec::new();
    for shard_path in shard_paths {
        let shard_name = Path::new(&shard_path)
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to extract file name"))?;
        let new_path = shards_path.join(shard_name);
        fs::rename(&shard_path, &new_path)?;
        new_shard_paths.push(new_path.to_string_lossy().to_string());
    }

    Ok(())
}
