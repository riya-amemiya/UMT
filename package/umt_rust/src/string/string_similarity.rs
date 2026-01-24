use super::levenshtein_distance::umt_levenshtein_distance;

/// Calculates the similarity between two strings as a percentage
///
/// Uses Levenshtein distance normalized by the length of the longer string
///
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
///
/// # Returns
/// Similarity score between 0.0 (completely different) and 1.0 (identical)
///
/// # Example
/// ```
/// use umt_rust::string::umt_string_similarity;
/// assert_eq!(umt_string_similarity("hello", "hello"), 1.0);
/// assert!(umt_string_similarity("hello", "hallo") > 0.5);
/// ```
#[inline]
pub fn umt_string_similarity(s1: &str, s2: &str) -> f64 {
    // Identical strings have 100% similarity
    if s1 == s2 {
        return 1.0;
    }

    // Empty strings handling
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    let distance = umt_levenshtein_distance(s1, s2);
    let max_length = s1.chars().count().max(s2.chars().count());

    // Calculate similarity as 1 - (distance / maxLength)
    1.0 - (distance as f64 / max_length as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_similarity_identical() {
        assert_eq!(umt_string_similarity("hello", "hello"), 1.0);
    }

    #[test]
    fn test_string_similarity_different() {
        let similarity = umt_string_similarity("hello", "hallo");
        assert!(similarity > 0.5);
        assert!(similarity < 1.0);
    }

    #[test]
    fn test_string_similarity_empty() {
        assert_eq!(umt_string_similarity("", "hello"), 0.0);
        assert_eq!(umt_string_similarity("hello", ""), 0.0);
        assert_eq!(umt_string_similarity("", ""), 1.0);
    }

    #[test]
    fn test_string_similarity_completely_different() {
        let similarity = umt_string_similarity("abc", "xyz");
        assert!(similarity < 0.5);
    }
}
