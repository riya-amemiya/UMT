use crate::internal::rng;

const DEFAULT_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Generates a random string
///
/// # Arguments
/// * `size` - Length of the random string
/// * `chars` - Optional string of characters to use for generating random string
///
/// # Returns
/// Random string
///
/// # Example
/// ```
/// use umt_rust::string::umt_random_string;
/// let s = umt_random_string(8, None);
/// assert_eq!(s.len(), 8);
/// ```
#[inline]
pub fn umt_random_string(size: usize, chars: Option<&str>) -> String {
    let char_set: Vec<char> = chars.unwrap_or(DEFAULT_CHARS).chars().collect();
    let length = char_set.len();

    if length == 0 {
        return String::new();
    }

    (0..size)
        .map(|_| char_set[rng::random_range_usize(0, length - 1)])
        .collect()
}

/// Generates a random string with default character set
#[inline]
pub fn umt_random_string_default(size: usize) -> String {
    umt_random_string(size, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_string_length() {
        let s = umt_random_string(8, None);
        assert_eq!(s.len(), 8);
    }

    #[test]
    fn test_random_string_custom_chars() {
        let s = umt_random_string(10, Some("abc"));
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| "abc".contains(c)));
    }

    #[test]
    fn test_random_string_empty_size() {
        let s = umt_random_string(0, None);
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_random_string_default() {
        let s = umt_random_string_default(16);
        assert_eq!(s.len(), 16);
    }
}
