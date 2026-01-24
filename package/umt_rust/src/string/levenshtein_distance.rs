/// Calculates the Levenshtein distance between two strings
/// Returns the minimum number of single-character edits (insertions, deletions, or substitutions)
///
/// # Arguments
///
/// * `string1` - First string to compare
/// * `string2` - Second string to compare
///
/// # Returns
///
/// The Levenshtein distance
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_levenshtein_distance;
///
/// assert_eq!(umt_levenshtein_distance("hello", "hello"), 0);
/// assert_eq!(umt_levenshtein_distance("cat", "bat"), 1);
/// assert_eq!(umt_levenshtein_distance("kitten", "sitting"), 3);
/// ```
#[inline]
pub fn umt_levenshtein_distance(string1: &str, string2: &str) -> usize {
    let chars1: Vec<char> = string1.chars().collect();
    let chars2: Vec<char> = string2.chars().collect();
    let length1 = chars1.len();
    let length2 = chars2.len();

    // Return the length of the other string if one is empty
    if length1 == 0 {
        return length2;
    }
    if length2 == 0 {
        return length1;
    }

    // Create a 2D array for dynamic programming
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; length2 + 1]; length1 + 1];

    // Initialize first column and row
    for i in 0..=length1 {
        matrix[i][0] = i;
    }
    for j in 0..=length2 {
        matrix[0][j] = j;
    }

    // Calculate distances
    for i in 1..=length1 {
        for j in 1..=length2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1) // deletion
                .min(matrix[i][j - 1] + 1) // insertion
                .min(matrix[i - 1][j - 1] + cost); // substitution
        }
    }

    matrix[length1][length2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_strings() {
        assert_eq!(umt_levenshtein_distance("hello", "hello"), 0);
        assert_eq!(umt_levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(umt_levenshtein_distance("", "hello"), 5);
        assert_eq!(umt_levenshtein_distance("hello", ""), 5);
    }

    #[test]
    fn test_single_character_differences() {
        assert_eq!(umt_levenshtein_distance("cat", "bat"), 1); // substitution
        assert_eq!(umt_levenshtein_distance("cat", "cats"), 1); // insertion
        assert_eq!(umt_levenshtein_distance("cats", "cat"), 1); // deletion
    }

    #[test]
    fn test_multiple_differences() {
        assert_eq!(umt_levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(umt_levenshtein_distance("saturday", "sunday"), 3);
    }

    #[test]
    fn test_completely_different_strings() {
        assert_eq!(umt_levenshtein_distance("abc", "xyz"), 3);
        assert_eq!(umt_levenshtein_distance("hello", "world"), 4);
    }

    #[test]
    fn test_case_sensitive() {
        assert_eq!(umt_levenshtein_distance("Hello", "hello"), 1);
        assert_eq!(umt_levenshtein_distance("ABC", "abc"), 3);
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(umt_levenshtein_distance("caf\u{E9}", "cafe"), 1);
        assert_eq!(umt_levenshtein_distance("\u{1F600}", "\u{1F601}"), 1);
        assert_eq!(
            umt_levenshtein_distance(
                "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}",
                "\u{3053}\u{3093}\u{3070}\u{3093}\u{306F}"
            ),
            2
        );
    }
}
