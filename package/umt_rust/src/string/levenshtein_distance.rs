/// Calculates the Levenshtein distance between two strings
///
/// Returns the minimum number of single-character edits (insertions, deletions, or substitutions)
///
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
///
/// # Returns
/// The Levenshtein distance
///
/// # Example
/// ```
/// use umt_rust::string::umt_levenshtein_distance;
/// assert_eq!(umt_levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(umt_levenshtein_distance("hello", "hello"), 0);
/// ```
#[inline]
pub fn umt_levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let (len1, len2) = (chars1.len(), chars2.len());

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    // Ensure chars1 is the shorter sequence to minimize space to O(min(n, m))
    if len1 > len2 {
        return umt_levenshtein_distance(s2, s1);
    }

    // Single-row optimization: only one row + a diagonal variable needed
    let mut row: Vec<usize> = (0..=len1).collect();

    for i in 1..=len2 {
        let mut prev_diagonal = row[0];
        row[0] = i;
        let c2 = chars2[i - 1];

        for j in 1..=len1 {
            let temp = row[j];
            let cost = if chars1[j - 1] == c2 { 0 } else { 1 };
            row[j] = (row[j] + 1) // deletion
                .min(row[j - 1] + 1) // insertion
                .min(prev_diagonal + cost); // substitution
            prev_diagonal = temp;
        }
    }

    row[len1]
}
