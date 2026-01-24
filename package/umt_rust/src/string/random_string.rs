use rand::Rng;

const DEFAULT_CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Generates a random string.
///
/// # Arguments
///
/// * `size` - Length of the random string (default: 8)
/// * `char_set` - String of characters to use for generating random string (default: alphanumeric)
///
/// # Returns
///
/// Random string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_random_string;
///
/// let s = umt_random_string(8, None);
/// assert_eq!(s.len(), 8);
/// ```
#[inline]
pub fn umt_random_string(size: usize, char_set: Option<&str>) -> String {
    let chars: Vec<char> = char_set.unwrap_or(DEFAULT_CHARSET).chars().collect();
    let length = chars.len();

    if length == 0 {
        return String::new();
    }

    let mut rng = rand::rng();
    let mut id = String::with_capacity(size);

    for _ in 0..size {
        let idx = rng.random_range(0..length);
        id.push(chars[idx]);
    }

    id
}

/// Initializes a function that generates random strings with a pre-configured character set.
///
/// # Arguments
///
/// * `char_set` - String of characters to use for generating random strings
///
/// # Returns
///
/// A function that generates random strings given a size
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_random_string_initialization;
///
/// let generator = umt_random_string_initialization(Some("abc123"));
/// let s = generator(5);
/// assert_eq!(s.len(), 5);
/// assert!(s.chars().all(|c| "abc123".contains(c)));
/// ```
#[inline]
pub fn umt_random_string_initialization(char_set: Option<&str>) -> impl Fn(usize) -> String {
    let chars: Vec<char> = char_set.unwrap_or(DEFAULT_CHARSET).chars().collect();

    move |size: usize| {
        if chars.is_empty() {
            return String::new();
        }

        let mut rng = rand::rng();
        let mut id = String::with_capacity(size);

        for _ in 0..size {
            let idx = rng.random_range(0..chars.len());
            id.push(chars[idx]);
        }

        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_length_and_characters() {
        let s = umt_random_string(8, None);
        assert_eq!(s.len(), 8);
        assert!(s
            .chars()
            .all(|c| "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".contains(c)));
    }

    #[test]
    fn test_custom_character_set() {
        let chars = "abc123";
        let s = umt_random_string(10, Some(chars));
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| chars.contains(c)));
    }

    #[test]
    fn test_specified_length() {
        let s = umt_random_string(20, None);
        assert_eq!(s.len(), 20);
    }

    #[test]
    fn test_initialization_default_charset() {
        let generator = umt_random_string_initialization(None);
        let result = generator(10);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_initialization_custom_charset() {
        let custom_char_set = "ABC123";
        let generator = umt_random_string_initialization(Some(custom_char_set));
        let result = generator(5);
        assert_eq!(result.len(), 5);
        assert!(result.chars().all(|c| custom_char_set.contains(c)));
    }
}
