/// Splits a string into chunks of the specified length.
///
/// Operates on Unicode scalar values, so characters outside the Basic
/// Multilingual Plane (e.g. emoji) are not split.
///
/// # Arguments
/// * `s` - The string to split
/// * `length` - The maximum length of each chunk
///
/// # Returns
/// A vector of string chunks
///
/// # Examples
/// ```
/// use umt_rust::string::umt_split_by_length;
/// assert_eq!(umt_split_by_length("abcdefg", 3), vec!["abc", "def", "g"]);
/// assert_eq!(umt_split_by_length("😀😁😂😃", 2), vec!["😀😁", "😂😃"]);
/// ```
pub fn umt_split_by_length(s: &str, length: usize) -> Vec<String> {
    if length == 0 {
        return Vec::new();
    }
    let chars: Vec<char> = s.chars().collect();
    chars
        .chunks(length)
        .map(|chunk| chunk.iter().collect())
        .collect()
}
