use crate::gf256::GF256;
use rand::Rng;

/// Represents a polynomial where the coefficients are elements of GF(2^8).
pub struct Polynomial {
    /// Coefficients of the polynomial, where each coefficient is an element of GF(2^8).
    pub coefficients: Vec<u8>,
}

impl Polynomial {
    /// Creates a new polynomial with a given intercept and random coefficients for the remaining terms.
    ///
    /// # Arguments
    ///
    /// * `intercept` - The constant term of the polynomial.
    /// * `degree` - The degree of the polynomial which determines the number of random coefficients to generate.
    ///
    /// # Returns
    ///
    /// A `Polynomial` with random coefficients and the specified intercept.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Polynomial::new(1, 3);
    /// assert_eq!(p.coefficients[0], 1); // intercept is the first coefficient
    /// assert!(p.coefficients.len() == 4); // degree 3 means 4 coefficients
    /// ```
    pub fn new(intercept: u8, degree: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Generate random coefficients and set the first one to the intercept.
        let coefficients: Vec<u8> = std::iter::once(intercept)
            .chain((0..degree).map(|_| rng.gen_range(0..=255)))
            .collect();

        Polynomial { coefficients }
    }

    /// Evaluates the polynomial at a given point `x` using Horner's method.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the polynomial.
    ///
    /// # Returns
    ///
    /// The value of the polynomial evaluated at `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Polynomial::new(1, 3); // let's assume it generates 1 + 2x + 3x^2 + 4x^3
    /// let value = p.evaluate(2); // evaluates 1 + 2*2 + 3*2^2 + 4*2^3 in GF(2^8)
    /// // Note: The actual result would depend on the GF(2^8) field arithmetic
    /// ```
    pub fn evaluate(&self, x: u8) -> u8 {
        self.coefficients
            .iter()
            .rev()
            .fold(0, |acc, &coeff| GF256::add(GF256::mult(acc, x), coeff))
    }
}

#[cfg(test)]
mod tests {
    use crate::{gf256::GF256, polynomial::Polynomial};

    #[test]
    fn test_polynomial_random() {
        let p = Polynomial::new(42, 2);
        assert_eq!(p.coefficients[0], 42);
    }

    #[test]
    fn test_polynomial_eval() {
        let p = Polynomial::new(42, 1);
        let mut out = p.evaluate(0);
        assert_eq!(out, 42);
        out = p.evaluate(1);
        let exp = GF256::add(42, GF256::mult(1, p.coefficients[1]));
        assert_eq!(out, exp);
    }
}
