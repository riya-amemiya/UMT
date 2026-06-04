/// Counts the number of non-overlapping occurrences of a substring.
///
/// Returns 0 when the search string is empty.
///
/// # Arguments
/// * `s` - The string to search within
/// * `search` - The substring to count
///
/// # Returns
/// The number of non-overlapping occurrences
///
/// # Examples
/// ```
/// use umt_rust::string::umt_count_occurrences;
/// assert_eq!(umt_count_occurrences("ababab", "ab"), 3);
/// assert_eq!(umt_count_occurrences("aaaa", "aa"), 2);
/// ```
#[inline]
pub fn umt_count_occurrences(s: &str, search: &str) -> usize {
    if search.is_empty() {
        return 0;
    }
    s.matches(search).count()
}
