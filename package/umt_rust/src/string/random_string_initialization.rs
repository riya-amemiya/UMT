use crate::string::umt_random_string;

/// Initializes a function that generates random strings.
///
/// # Arguments
/// * `chars` - Optional string of characters to use for generating random strings
///
/// # Returns
/// A function that generates random strings (size: usize) -> String
///
/// # Example
/// ```
/// use umt_rust::string::umt_random_string_initialization;
///
/// let random_func = umt_random_string_initialization(Some("ABC"));
/// let s = random_func(5);
/// assert_eq!(s.len(), 5);
/// assert!(s.chars().all(|c| "ABC".contains(c)));
/// ```
pub fn umt_random_string_initialization(chars: Option<&str>) -> impl Fn(usize) -> String {
    let chars = chars.map(|s| s.to_string());
    move |size| umt_random_string(size, chars.as_deref())
}
