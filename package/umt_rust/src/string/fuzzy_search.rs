use crate::string::umt_levenshtein_distance;

/// A result from fuzzy search containing the matched item and its similarity score
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzySearchResult {
    pub item: String,
    pub score: f64,
}

/// Perform fuzzy string matching on an array of strings
///
/// # Arguments
///
/// * `query` - The search query
/// * `items` - Array of strings to search in
/// * `threshold` - Similarity threshold (0-1) for matching (default: 0.6)
///
/// # Returns
///
/// Array of matching items with their similarity scores, sorted by best match
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_fuzzy_search;
///
/// let result = umt_fuzzy_search("hello", &["hello", "world", "helo", "help"], 0.6);
/// assert_eq!(result.len(), 3);
/// assert_eq!(result[0].item, "hello");
/// assert_eq!(result[0].score, 1.0);
/// ```
#[inline]
pub fn umt_fuzzy_search(query: &str, items: &[&str], threshold: f64) -> Vec<FuzzySearchResult> {
    if query.is_empty() {
        return vec![];
    }

    let query_lower = query.to_lowercase();
    let mut results: Vec<FuzzySearchResult> = Vec::new();

    for &item in items {
        let item_lower = item.to_lowercase();
        let distance = umt_levenshtein_distance(&query_lower, &item_lower);
        let max_length = query.chars().count().max(item.chars().count());
        let score = 1.0 - (distance as f64 / max_length as f64);

        if score >= threshold {
            results.push(FuzzySearchResult {
                item: item.to_string(),
                score,
            });
        }
    }

    // Sort by score descending (using quicksort-like approach)
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_search_jsdoc_example() {
        let result = umt_fuzzy_search("hello", &["hello", "world", "helo", "help"], 0.6);
        assert!(result.iter().any(|r| r.item == "hello" && r.score == 1.0));
        assert!(result.iter().any(|r| r.item == "helo" && (r.score - 0.8).abs() < 0.001));
        assert!(result.iter().any(|r| r.item == "help" && (r.score - 0.6).abs() < 0.001));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_exact_matches() {
        let result = umt_fuzzy_search("test", &["test", "best", "rest"], 0.6);
        assert_eq!(result[0].item, "test");
        assert_eq!(result[0].score, 1.0);
    }

    #[test]
    fn test_sorted_by_score_descending() {
        let result = umt_fuzzy_search("test", &["test", "tests", "testing"], 0.6);
        for i in 0..result.len() - 1 {
            assert!(result[i].score >= result[i + 1].score);
        }
    }

    #[test]
    fn test_custom_threshold() {
        let high_threshold = umt_fuzzy_search("hello", &["hello", "helo", "help"], 0.9);
        let low_threshold = umt_fuzzy_search("hello", &["hello", "helo", "help"], 0.3);
        assert!(high_threshold.len() <= low_threshold.len());
    }

    #[test]
    fn test_case_insensitive() {
        let result = umt_fuzzy_search("Hello", &["HELLO", "hello", "Hello"], 0.6);
        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|r| r.score == 1.0));
    }

    #[test]
    fn test_empty_query() {
        let result = umt_fuzzy_search("", &["hello", "world"], 0.6);
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_items() {
        let result = umt_fuzzy_search("hello", &[], 0.6);
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_below_threshold() {
        let result = umt_fuzzy_search("hello", &["world", "xyz"], 0.8);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_character_matches() {
        let result = umt_fuzzy_search("a", &["a", "b", "ab"], 0.5);
        assert!(result.iter().any(|r| r.item == "a" && r.score == 1.0));
    }

    #[test]
    fn test_special_characters() {
        let result = umt_fuzzy_search("hello!", &["hello!", "hello"], 0.6);
        assert_eq!(result[0].item, "hello!");
        assert_eq!(result[0].score, 1.0);
    }
}
