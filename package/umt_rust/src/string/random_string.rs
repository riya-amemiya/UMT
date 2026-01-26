use rand::Rng;

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

    let mut rng = rand::rng();
    (0..size)
        .map(|_| char_set[rng.random_range(0..length)])
        .collect()
}

/// Generates a random string with default character set
#[inline]
pub fn umt_random_string_default(size: usize) -> String {
    umt_random_string(size, None)
}
