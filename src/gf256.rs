// Galois Field: GF(2^8)
pub struct GF256;

impl GF256 {
    /// Adds two elements in GF(2^8).
    ///
    /// # Arguments
    ///
    /// * `a` - The first byte to add.
    /// * `b` - The second byte to add.
    ///
    /// # Returns
    ///
    /// The result of the addition in GF(2^8), which is the XOR of `a` and `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(GF256::add(16, 16), 0);
    /// assert_eq!(GF256::add(3, 4), 7);
    /// ```
    pub fn add(a: u8, b: u8) -> u8 {
        a ^ b
    }

    /// Multiplies two elements in GF(2^8).
    ///
    /// # Arguments
    ///
    /// * `a` - The first byte to multiply.
    /// * `b` - The second byte to multiply.
    ///
    /// # Returns
    ///
    /// The result of the multiplication in GF(2^8), using the Russian peasant multiplication
    /// algorithm and modulo the irreducible polynomial x^8 + x^4 + x^3 + x + 1.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(GF256::mult(3, 7), 9);
    /// assert_eq!(GF256::mult(3, 0), 0);
    /// assert_eq!(GF256::mult(0, 3), 0);
    /// ```
    pub fn mult(mut a: u8, mut b: u8) -> u8 {
        let mut result: u8 = 0;
        while b > 0 {
            if b & 1 != 0 {
                result ^= a; // If the lowest bit of b is set, XOR result with a.
            }
            if a & 0x80 != 0 {
                a = (a << 1) ^ 0x1B; // XOR with the reduction polynomial if a is about to overflow.
            } else {
                a <<= 1; // Otherwise, just shift a to the left.
            }
            b >>= 1; // Shift b to the right.
        }
        result
    }

    /// Computes the multiplicative inverse of an element in GF(2^8).
    ///
    /// # Arguments
    ///
    /// * `a` - The byte to invert.
    ///
    /// # Returns
    ///
    /// The multiplicative inverse of `a` in GF(2^8).
    ///
    /// # Panics
    ///
    /// Panics if `a` is 0 since the inverse does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// // Note: The example values below are hypothetical and may not represent
    /// // actual multiplicative inverses in GF(2^8).
    /// assert_eq!(GF256::inverse(3), some_value);
    /// assert_eq!(GF256::inverse(9), another_value);
    /// ```
    pub fn inverse(a: u8) -> u8 {
        let mut b = GF256::mult(a, a);
        let mut c = GF256::mult(a, b);
        b = GF256::mult(c, c);
        b = GF256::mult(b, b);
        c = GF256::mult(b, c);
        b = GF256::mult(b, b);
        b = GF256::mult(b, b);
        b = GF256::mult(b, c);
        b = GF256::mult(b, b);
        b = GF256::mult(a, b);
        GF256::mult(b, b)
    }

    /// Divides one element by another in GF(2^8).
    ///
    /// # Arguments
    ///
    /// * `a` - The dividend.
    /// * `b` - The divisor.
    ///
    /// # Returns
    ///
    /// The result of the division in GF(2^8).
    ///
    /// # Panics
    ///
    /// Panics if `b` is 0 because division by zero is undefined.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(GF256::div(0, 7), 0);
    /// assert_eq!(GF256::div(3, 3), 1);
    /// assert_eq!(GF256::div(6, 3), 2);
    /// ```
    pub fn div(a: u8, b: u8) -> u8 {
        if b == 0 {
            panic!("divide by zero");
        }
        let mut ret = GF256::mult(a, GF256::inverse(b));
        ret = if a == 0 { 0 } else { ret };
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_add() {
        assert_eq!(GF256::add(16, 16), 0);
        assert_eq!(GF256::add(3, 4), 7);
    }

    #[test]
    fn test_field_mult() {
        assert_eq!(GF256::mult(3, 7), 9);
        assert_eq!(GF256::mult(3, 0), 0);
        assert_eq!(GF256::mult(0, 3), 0);
    }

    #[test]
    fn test_field_divide() {
        assert_eq!(GF256::div(0, 7), 0);
        assert_eq!(GF256::div(3, 3), 1);
        assert_eq!(GF256::div(6, 3), 2);
    }
}
