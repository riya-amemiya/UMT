use crate::string::umt_levenshtein_distance;

/// Calculates the similarity between two strings as a percentage
/// Uses Levenshtein distance normalized by the length of the longer string
///
/// # Arguments
///
/// * `string1` - First string to compare
/// * `string2` - Second string to compare
///
/// # Returns
///
/// Similarity score between 0 (completely different) and 1 (identical)
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_string_similarity;
///
/// assert_eq!(umt_string_similarity("hello", "hello"), 1.0);
/// assert!((umt_string_similarity("cat", "bat") - 0.667).abs() < 0.001);
/// ```
#[inline]
pub fn umt_string_similarity(string1: &str, string2: &str) -> f64 {
    // Identical strings have 100% similarity
    if string1 == string2 {
        return 1.0;
    }

    let len1 = string1.chars().count();
    let len2 = string2.chars().count();

    // Empty strings handling
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let distance = umt_levenshtein_distance(string1, string2);
    let max_length = len1.max(len2);

    // Calculate similarity as 1 - (distance / maxLength)
    1.0 - (distance as f64 / max_length as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_strings() {
        assert_eq!(umt_string_similarity("hello", "hello"), 1.0);
        assert_eq!(umt_string_similarity("world", "world"), 1.0);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(umt_string_similarity("", "hello"), 0.0);
        assert_eq!(umt_string_similarity("hello", ""), 0.0);
        assert_eq!(umt_string_similarity("", ""), 1.0); // both empty are identical
    }

    #[test]
    fn test_similar_strings() {
        let similarity1 = umt_string_similarity("cat", "bat");
        assert!((similarity1 - 0.667).abs() < 0.001); // 1 - (1/3)

        let similarity2 = umt_string_similarity("kitten", "sitting");
        assert!((similarity2 - 0.571).abs() < 0.001); // 1 - (3/7)
    }

    #[test]
    fn test_completely_different_strings() {
        let similarity = umt_string_similarity("abc", "xyz");
        assert_eq!(similarity, 0.0); // 1 - (3/3)
    }

    #[test]
    fn test_different_lengths() {
        let similarity1 = umt_string_similarity("cat", "cats");
        assert_eq!(similarity1, 0.75); // 1 - (1/4)

        let similarity2 = umt_string_similarity("hello", "helo");
        assert_eq!(similarity2, 0.8); // 1 - (1/5)
    }

    #[test]
    fn test_case_sensitive() {
        let similarity = umt_string_similarity("Hello", "hello");
        assert_eq!(similarity, 0.8); // 1 - (1/5)
    }

    #[test]
    fn test_unicode_characters() {
        let similarity1 = umt_string_similarity("caf\u{E9}", "cafe");
        assert_eq!(similarity1, 0.75); // 1 - (1/4)
    }

    #[test]
    fn test_values_between_0_and_1() {
        let test_cases = vec![
            ("hello", "world"),
            ("test", "testing"),
            ("a", "b"),
            ("similar", "similarity"),
        ];

        for (str1, str2) in test_cases {
            let similarity = umt_string_similarity(str1, str2);
            assert!(similarity >= 0.0);
            assert!(similarity <= 1.0);
        }
    }
}
