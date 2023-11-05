use crate::{gf256::GF256, polynomial::Polynomial};
use rand::seq::IteratorRandom;
use std::collections::HashSet;

type Shares = Vec<Vec<u8>>;

/// Interpolates a polynomial at a given x-coordinate using Lagrange interpolation
/// in the finite field GF(2^8).
///
/// # Arguments
///
/// * `x_samples` - A slice of x-coordinates from the shares.
/// * `y_samples` - A slice of y-coordinates from the shares.
/// * `x` - The x-coordinate at which to interpolate the polynomial.
///
/// # Returns
///
/// The interpolated value at the given x-coordinate.
///
/// # Examples
///
/// ```
/// let x_samples = vec![1, 2, 3];
/// let y_samples = vec![5, 8, 15]; // for a polynomial like f(x) = 2x + 3
/// let y_at_4 = interpolate_polynomial(&x_samples, &y_samples, 4);
/// // Assuming GF(2^8) arithmetic, the result would be the evaluation at x = 4.
/// ```
fn interpolate_polynomial(x_samples: &[u8], y_samples: &[u8], x: u8) -> u8 {
    x_samples
        .iter()
        .enumerate()
        .map(|(i, &xi)| {
            let basis = x_samples
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .map(|(_, &xj)| {
                    let num = GF256::add(x, xj);
                    let denom = GF256::add(xi, xj);
                    GF256::div(num, denom)
                })
                .fold(1, GF256::mult);

            GF256::mult(y_samples[i], basis)
        })
        .fold(0, GF256::add)
}

/// Splits a secret into a given number of parts, with a defined threshold of parts
/// needed to reconstruct the secret using Shamir's Secret Sharing scheme.
///
/// # Arguments
///
/// * `secret` - A byte slice representing the secret to be split.
/// * `parts` - The number of shares to produce.
/// * `threshold` - The minimum number of shares required to reconstruct the secret.
///
/// # Returns
///
/// A vector of shares, each of which is a vector of bytes.
///
/// # Panics
///
/// The function panics if the parts are fewer than the threshold, exceed 255,
/// if the threshold is less than 2, exceeds 255, or if the secret is empty.
///
/// # Examples
///
/// ```
/// let secret = b"Rust secret".to_vec();
/// let shares = split(&secret, 5, 3); // split the secret into 5 parts, 3 needed to reconstruct
/// // Each share should contain a piece of the secret and an identifier.
/// ```
pub fn split(secret: &[u8], parts: usize, threshold: usize) -> Shares {
    match () {
        _ if parts < threshold => panic!("parts cannot be less than threshold"),
        _ if parts > 255 => panic!("parts cannot exceed 255"),
        _ if threshold < 2 => panic!("threshold must be at least 2"),
        _ if threshold > 255 => panic!("threshold cannot exceed 255"),
        _ if secret.is_empty() => panic!("cannot split an empty secret"),
        _ => (),
    }

    let mut rng = rand::thread_rng();
    let x_coordinates: Vec<u8> = (1..=255_u8).choose_multiple(&mut rng, parts);

    // Create empty shares with preallocated space
    let mut shares: Shares = x_coordinates
        .iter()
        .map(|&x| {
            let mut buffer = vec![0; secret.len() + 1];
            buffer[secret.len()] = x;
            buffer
        })
        .collect();

    // Fill shares with evaluated polynomial values
    secret.iter().enumerate().for_each(|(idx, &value)| {
        let polynomial = Polynomial::new(value, threshold - 1);
        x_coordinates.iter().enumerate().for_each(|(i, &x)| {
            shares[i][idx] = polynomial.evaluate(x);
        });
    });

    shares
}

/// Combines shares to reconstruct a secret using Shamir's Secret Sharing scheme.
///
/// # Arguments
///
/// * `parts` - A vector of shares where each share is a vector of bytes.
///
/// # Returns
///
/// A vector of bytes representing the reconstructed secret.
///
/// # Panics
///
/// The function panics if less than two shares are provided, if all shares
/// are not the same length, at least two bytes long, or if duplicate shares are detected.
///
/// # Examples
///
/// ```
/// let shares = split(&b"Rust secret".to_vec(), 5, 3); // Assuming `split` was successful
/// let reconstructed_secret = combine(shares); // Combine the shares to reconstruct the secret
/// assert_eq!(reconstructed_secret, b"Rust secret".to_vec());
/// ```
pub fn combine(parts: Shares) -> Vec<u8> {
    let parts_len = parts.len();
    if parts_len < 2 {
        panic!("less than two parts cannot be used to reconstruct the secret");
    }

    // Ensure all parts are the same length and have at least two bytes
    let first_part_len = parts.get(0).map_or(0, Vec::len);
    if first_part_len < 2 || parts.iter().any(|part| part.len() != first_part_len) {
        panic!("all parts must be at least two bytes and the same length");
    }

    // Create a hash set to check for duplicate x-coordinates
    let mut check_map = HashSet::new();
    let x_samples: Vec<u8> = parts
        .iter()
        .map(|part| {
            let x = *part.last().expect("part is non-empty");
            if !check_map.insert(x) {
                panic!("duplicate part detected");
            }
            x
        })
        .collect();

    // Initialize the secret vector
    let mut secret = vec![0; first_part_len - 1];

    // Interpolate the polynomial at 0 for each byte of the secret
    for idx in 0..secret.len() {
        let y_samples: Vec<u8> = parts.iter().map(|part| part[idx]).collect();
        secret[idx] = interpolate_polynomial(&x_samples, &y_samples, 0);
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_split_invalid() {
        let secret = b"test".to_vec();

        let _ = split(&secret, 0, 0);
        let _ = split(&secret, 2, 3);
        let _ = split(&secret, 1000, 3);
        let _ = split(&secret, 10, 1);
        let _ = split(&[], 3, 2);
    }

    #[test]
    fn test_split() {
        let secret = b"test".to_vec();

        let out = split(&secret, 5, 3);
        assert_eq!(out.len(), 5);

        out.iter().for_each(|share| {
            assert_eq!(share.len(), secret.len() + 1);
        });
    }

    #[test]
    #[should_panic]
    fn test_combine_invalid() {
        let _ = combine(vec![]);

        let parts = [b"foo".to_vec(), b"ba".to_vec()];
        let _ = combine(parts.to_vec());

        let short_parts = [b"f".to_vec(), b"b".to_vec()];
        let _ = combine(short_parts.to_vec());

        let same_parts = [b"foo".to_vec(), b"foo".to_vec()];
        let _ = combine(same_parts.to_vec());
    }

    #[test]
    fn test_combine() {
        let secret = b"test".to_vec();
        let out = split(&secret, 5, 3);

        for i in 0..5 {
            for j in 0..5 {
                if j == i {
                    continue;
                }
                for k in 0..5 {
                    if k == i || k == j {
                        continue;
                    }
                    let parts = vec![out[i].clone(), out[j].clone(), out[k].clone()];
                    let recomb = combine(parts);
                    assert_eq!(recomb, secret);
                }
            }
        }
    }

    #[test]
    fn test_interpolate_rand() {
        for i in 0..255 {
            let p = Polynomial::new(i, 2);
            let x_vals = vec![1, 2, 3];
            let y_vals = vec![p.evaluate(1), p.evaluate(2), p.evaluate(3)];
            let out = interpolate_polynomial(&x_vals, &y_vals, 0);
            assert_eq!(out, i);
        }
    }
}
