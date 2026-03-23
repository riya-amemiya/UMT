/// Joins multiple path segments into a single path,
/// normalizing slashes between segments.
///
/// Leading slash of the first segment is preserved.
/// Trailing slash of the last segment is preserved.
/// All intermediate slashes are normalized to a single slash.
///
/// # Arguments
///
/// * `segments` - The path segments to join
///
/// # Returns
///
/// The joined and normalized path.
///
/// # Example
///
/// ```
/// use umt_rust::url::umt_join_path;
///
/// assert_eq!(umt_join_path(&["https://example.com/", "/api/", "/users"]), "https://example.com/api/users");
/// assert_eq!(umt_join_path(&["/a/", "/b/", "/c"]), "/a/b/c");
/// assert_eq!(umt_join_path(&["a", "b", "c"]), "a/b/c");
/// ```
pub fn umt_join_path(segments: &[&str]) -> String {
    if segments.is_empty() {
        return String::new();
    }

    let mut normalized: Vec<&str> = Vec::new();

    for (index, segment) in segments.iter().enumerate() {
        let mut s = *segment;

        if index > 0 {
            s = s.trim_start_matches('/');
        }

        if index < segments.len() - 1 {
            s = s.trim_end_matches('/');
        }

        if !s.is_empty() {
            normalized.push(s);
        }
    }

    normalized.join("/")
}
