//! Prime number checking utility

/// Determines if a number is prime
///
/// # Arguments
/// * `n` - Number to check (must be an integer)
///
/// # Returns
/// true if the number is prime, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_prime_number;
///
/// assert!(umt_is_prime_number(2));
/// assert!(umt_is_prime_number(17));
/// assert!(!umt_is_prime_number(4));
/// assert!(!umt_is_prime_number(1));
/// assert!(!umt_is_prime_number(-3));
/// ```
#[inline]
pub fn umt_is_prime_number(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i64;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

/// Determines if a usize is prime
///
/// # Arguments
/// * `n` - Number to check
///
/// # Returns
/// true if the number is prime, false otherwise
#[inline]
pub fn umt_is_prime_number_usize(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_number() {
        assert!(umt_is_prime_number(2));
        assert!(umt_is_prime_number(3));
        assert!(umt_is_prime_number(5));
        assert!(umt_is_prime_number(7));
        assert!(umt_is_prime_number(11));
        assert!(umt_is_prime_number(13));
        assert!(umt_is_prime_number(17));
        assert!(umt_is_prime_number(97));

        assert!(!umt_is_prime_number(0));
        assert!(!umt_is_prime_number(1));
        assert!(!umt_is_prime_number(4));
        assert!(!umt_is_prime_number(6));
        assert!(!umt_is_prime_number(8));
        assert!(!umt_is_prime_number(9));
        assert!(!umt_is_prime_number(100));
        assert!(!umt_is_prime_number(-3));
    }

    #[test]
    fn test_is_prime_number_usize() {
        assert!(umt_is_prime_number_usize(2));
        assert!(umt_is_prime_number_usize(17));
        assert!(!umt_is_prime_number_usize(4));
        assert!(!umt_is_prime_number_usize(1));
    }
}
