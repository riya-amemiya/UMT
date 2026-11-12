/// Calculates the optimal string alignment (restricted Damerau-Levenshtein) distance
///
/// Insertions, deletions, substitutions and transpositions of two adjacent
/// characters each count as one edit, and no substring is edited twice.
/// Characters are compared as Unicode scalar values, like `umt_levenshtein_distance`.
/// Uses three rows sized by the shorter string, so space is O(min(N, M)).
///
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
///
/// # Returns
/// The optimal string alignment distance
///
/// # Example
/// ```
/// use umt_rust::string::umt_damerau_levenshtein_distance;
/// assert_eq!(umt_damerau_levenshtein_distance("ca", "ac"), 1);
/// assert_eq!(umt_damerau_levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(umt_damerau_levenshtein_distance("ca", "abc"), 3);
/// ```
pub fn umt_damerau_levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    let mut short: Vec<char> = s1.chars().collect();
    let mut long: Vec<char> = s2.chars().collect();
    if short.len() > long.len() {
        std::mem::swap(&mut short, &mut long);
    }
    let (short_len, long_len) = (short.len(), long.len());

    if short_len == 0 {
        return long_len;
    }

    let mut rows: [Vec<usize>; 3] = [
        (0..=short_len).collect(),
        vec![0; short_len + 1],
        vec![0; short_len + 1],
    ];

    for i in 1..=long_len {
        let [before_previous, previous, current] = rotate(&mut rows, i);
        let long_char = long[i - 1];
        current[0] = i;

        for j in 1..=short_len {
            let short_char = short[j - 1];
            let cost = if short_char == long_char { 0 } else { 1 };
            let mut best = (previous[j] + 1)
                .min(current[j - 1] + 1)
                .min(previous[j - 1] + cost);
            if i > 1 && j > 1 && long_char == short[j - 2] && long[i - 2] == short_char {
                best = best.min(before_previous[j - 2] + 1);
            }
            current[j] = best;
        }
    }

    rows[long_len % 3][short_len]
}

fn rotate(rows: &mut [Vec<usize>; 3], i: usize) -> [&mut Vec<usize>; 3] {
    let [a, b, c] = rows;
    match i % 3 {
        0 => [b, c, a],
        1 => [c, a, b],
        _ => [a, b, c],
    }
}
