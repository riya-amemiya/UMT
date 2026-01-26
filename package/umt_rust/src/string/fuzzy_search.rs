use super::levenshtein_distance::umt_levenshtein_distance;

/// Result of a fuzzy search match
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzyMatch {
    pub item: String,
    pub score: f64,
}

/// Perform fuzzy string matching on an array of strings
///
/// # Arguments
/// * `query` - The search query
/// * `items` - Array of strings to search in
/// * `threshold` - Similarity threshold (0.0-1.0) for matching (default: 0.6)
///
/// # Returns
/// Array of matching items with their similarity scores, sorted by best match
///
/// # Example
/// ```
/// use umt_rust::string::umt_fuzzy_search;
/// let results = umt_fuzzy_search("hello", &["hello", "world", "helo", "help"], 0.6);
/// assert_eq!(results[0].item, "hello");
/// assert_eq!(results[0].score, 1.0);
/// ```
#[inline]
pub fn umt_fuzzy_search(query: &str, items: &[&str], threshold: f64) -> Vec<FuzzyMatch> {
    if query.is_empty() {
        return vec![];
    }

    let query_lower = query.to_lowercase();
    let mut results: Vec<FuzzyMatch> = items
        .iter()
        .filter_map(|&item| {
            let item_lower = item.to_lowercase();
            let distance = umt_levenshtein_distance(&query_lower, &item_lower);
            let max_length = query.chars().count().max(item.chars().count());
            let score = 1.0 - (distance as f64 / max_length as f64);

            if score >= threshold {
                Some(FuzzyMatch {
                    item: item.to_string(),
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results
}

/// Perform fuzzy search with default threshold of 0.6
#[inline]
pub fn umt_fuzzy_search_default(query: &str, items: &[&str]) -> Vec<FuzzyMatch> {
    umt_fuzzy_search(query, items, 0.6)
}
