/// Converts a number to a string representation in the specified base.
///
/// # Arguments
///
/// * `value` - The number to convert.
/// * `radix` - The base to convert to (2-36).
///
/// # Returns
///
/// String representation of the number in the specified base.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_to_base_n;
///
/// assert_eq!(umt_to_base_n(10, 2), "1010"); // binary
/// assert_eq!(umt_to_base_n(15, 16), "f"); // hexadecimal
/// assert_eq!(umt_to_base_n(255, 16), "ff"); // hexadecimal
/// ```
pub fn umt_to_base_n(value: i64, radix: u32) -> String {
    if radix < 2 || radix > 36 {
        return String::from("Invalid radix");
    }

    if value == 0 {
        return String::from("0");
    }

    let negative = value < 0;
    let mut n = value.abs();
    let mut result = String::new();

    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

    while n > 0 {
        let digit = (n % radix as i64) as usize;
        result.insert(0, digits.chars().nth(digit).unwrap());
        n /= radix as i64;
    }

    if negative {
        result.insert(0, '-');
    }

    result
}

/// Converts a number to binary string representation.
///
/// # Arguments
///
/// * `value` - The number to convert.
///
/// # Returns
///
/// Binary string representation of the number.
#[inline]
pub fn umt_to_binary(value: i64) -> String {
    umt_to_base_n(value, 2)
}

/// Converts a number to hexadecimal string representation.
///
/// # Arguments
///
/// * `value` - The number to convert.
///
/// # Returns
///
/// Hexadecimal string representation of the number.
#[inline]
pub fn umt_to_hex(value: i64) -> String {
    umt_to_base_n(value, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base_n_binary() {
        assert_eq!(umt_to_base_n(10, 2), "1010");
    }

    #[test]
    fn test_to_base_n_hex() {
        assert_eq!(umt_to_base_n(15, 16), "f");
        assert_eq!(umt_to_base_n(255, 16), "ff");
    }

    #[test]
    fn test_to_base_n_octal() {
        assert_eq!(umt_to_base_n(8, 8), "10");
    }

    #[test]
    fn test_to_base_n_zero() {
        assert_eq!(umt_to_base_n(0, 2), "0");
        assert_eq!(umt_to_base_n(0, 16), "0");
    }

    #[test]
    fn test_to_base_n_negative() {
        assert_eq!(umt_to_base_n(-10, 2), "-1010");
    }

    #[test]
    fn test_to_binary() {
        assert_eq!(umt_to_binary(10), "1010");
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(umt_to_hex(255), "ff");
    }

    #[test]
    fn test_to_base_n_invalid_radix() {
        assert_eq!(umt_to_base_n(10, 1), "Invalid radix");
        assert_eq!(umt_to_base_n(10, 37), "Invalid radix");
    }
}
