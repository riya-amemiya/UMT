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
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    // Use single row optimization for space efficiency
    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row: Vec<usize> = vec![0; len2 + 1];

    for i in 1..=len1 {
        curr_row[0] = i;
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            curr_row[j] = (prev_row[j] + 1) // deletion
                .min(curr_row[j - 1] + 1) // insertion
                .min(prev_row[j - 1] + cost); // substitution
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[len2]
}
