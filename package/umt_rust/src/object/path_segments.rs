/// Splits a dot-separated path string into its segments.
///
/// Mirrors the TypeScript `pathSegments` helper used by `get`, `set` and
/// `unflattenObject`. A string is split on `.`; an empty string yields a
/// single empty segment (matching JavaScript's `"".split(".")`).
///
/// # Arguments
///
/// * `path` - The dot-separated path string.
///
/// # Returns
///
/// A vector of path segments.
///
/// # Examples
///
/// ```
/// use umt_rust::object::umt_path_segments;
///
/// assert_eq!(umt_path_segments("a.b.c"), vec!["a", "b", "c"]);
/// assert_eq!(umt_path_segments("a"), vec!["a"]);
/// assert_eq!(umt_path_segments(""), vec![""]);
/// ```
pub fn umt_path_segments(path: &str) -> Vec<String> {
    path.split('.').map(|segment| segment.to_string()).collect()
}
