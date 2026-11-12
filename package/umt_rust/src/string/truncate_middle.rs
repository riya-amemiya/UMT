/// Truncate a string in the middle, keeping its head and tail
///
/// The result length (in characters, like `umt_truncate`) is at most `length`.
/// When `length` is not larger than the separator, the separator is cut to `length`.
///
/// # Arguments
/// * `s` - The string to truncate
/// * `length` - The maximum length of the result
/// * `separator` - The text placed between head and tail (default: "...")
///
/// # Returns
/// The truncated string
///
/// # Example
/// ```
/// use umt_rust::string::umt_truncate_middle;
/// assert_eq!(umt_truncate_middle("Hello World", 8, "..."), "Hel...ld");
/// assert_eq!(umt_truncate_middle("Hello World", 7, "~"), "Hel~rld");
/// assert_eq!(umt_truncate_middle("Hello", 10, "..."), "Hello");
/// assert_eq!(umt_truncate_middle("Hello World", 2, "..."), "..");
/// ```
pub fn umt_truncate_middle(s: &str, length: usize, separator: &str) -> String {
    let char_count = s.chars().count();
    if char_count <= length {
        return s.to_string();
    }
    let separator_count = separator.chars().count();
    if length <= separator_count {
        return separator.chars().take(length).collect();
    }
    let available = length - separator_count;
    let head_length = available.div_ceil(2);
    let tail_length = available - head_length;
    let mut result: String = s.chars().take(head_length).collect();
    result.push_str(separator);
    result.extend(s.chars().skip(char_count - tail_length));
    result
}

/// Truncate a string in the middle with default separator "..."
pub fn umt_truncate_middle_default(s: &str, length: usize) -> String {
    umt_truncate_middle(s, length, "...")
}
