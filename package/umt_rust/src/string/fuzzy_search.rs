/// Result of a fuzzy search match
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzyMatch {
    pub item: String,
    pub score: f64,
}

fn bounded_distance(
    first: &[char],
    second: &[char],
    max_allowed_distance: f64,
    row: &mut Vec<usize>,
) -> Option<usize> {
    let (short, long) = if first.len() > second.len() {
        (second, first)
    } else {
        (first, second)
    };

    row.clear();
    row.extend(0..=short.len());

    for (row_index, &long_char) in long.iter().enumerate() {
        let mut previous_diagonal = row[0];
        row[0] = row_index + 1;
        let mut min_distance_in_row = row[0];

        for (column_index, &short_char) in short.iter().enumerate() {
            let above = row[column_index + 1];
            let cost = usize::from(short_char != long_char);
            let value = (above + 1)
                .min(row[column_index] + 1)
                .min(previous_diagonal + cost);
            row[column_index + 1] = value;
            previous_diagonal = above;
            min_distance_in_row = min_distance_in_row.min(value);
        }

        if min_distance_in_row as f64 > max_allowed_distance {
            return None;
        }
    }

    let distance = row[short.len()];
    if distance as f64 > max_allowed_distance {
        None
    } else {
        Some(distance)
    }
}

/// Perform fuzzy string matching on an array of strings
///
/// An item matches when its case-insensitive Levenshtein distance to the query
/// is at most `floor(max_length * (1 - threshold))`, where `max_length` is the
/// longer of the two lengths in characters.
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
    let query_length = query.chars().count();
    if query_length == 0 {
        return vec![];
    }

    let lower_query: Vec<char> = query.to_lowercase().chars().take(query_length).collect();
    let mut lower_item: Vec<char> = Vec::new();
    let mut row: Vec<usize> = Vec::new();
    let mut results: Vec<FuzzyMatch> = Vec::new();

    for &item in items {
        let item_length = item.chars().count();
        let max_length = item_length.max(query_length);
        let max_allowed_distance = max_length as f64 * (1.0 - threshold);

        if query_length.abs_diff(item_length) as f64 > max_allowed_distance {
            continue;
        }

        lower_item.clear();
        lower_item.extend(item.to_lowercase().chars().take(item_length));

        let Some(distance) =
            bounded_distance(&lower_query, &lower_item, max_allowed_distance, &mut row)
        else {
            continue;
        };

        results.push(FuzzyMatch {
            item: item.to_string(),
            score: 1.0 - (distance as f64 / max_length as f64),
        });
    }

    // Sort by score descending; treat NaN as less than any valid score
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

/// Perform fuzzy search with default threshold of 0.6
#[inline]
pub fn umt_fuzzy_search_default(query: &str, items: &[&str]) -> Vec<FuzzyMatch> {
    umt_fuzzy_search(query, items, 0.6)
}
