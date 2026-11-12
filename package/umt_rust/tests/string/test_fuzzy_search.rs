use umt_rust::string::{FuzzyMatch, umt_fuzzy_search, umt_fuzzy_search_default};

#[test]
fn test_perform_fuzzy_string_matching_according_to_jsdoc_example() {
    let result = umt_fuzzy_search("hello", &["hello", "world", "helo", "help"], 0.6);
    assert!(result.iter().any(|m| m.item == "hello" && m.score == 1.0));
    assert!(
        result
            .iter()
            .any(|m| m.item == "helo" && (m.score - 0.8).abs() < 0.001)
    );
    assert!(
        result
            .iter()
            .any(|m| m.item == "help" && (m.score - 0.6).abs() < 0.001)
    );
    assert_eq!(result.len(), 3);
}

#[test]
fn test_find_exact_matches() {
    let result = umt_fuzzy_search("test", &["test", "best", "rest"], 0.6);
    assert_eq!(result[0].item, "test");
    assert_eq!(result[0].score, 1.0);
}

#[test]
fn test_sort_results_by_score_descending() {
    let result = umt_fuzzy_search("test", &["test", "tests", "testing"], 0.6);
    for i in 0..result.len().saturating_sub(1) {
        assert!(result[i].score >= result[i + 1].score);
    }
}

#[test]
fn test_use_custom_threshold() {
    let high_threshold = umt_fuzzy_search("hello", &["hello", "helo", "help"], 0.9);
    let low_threshold = umt_fuzzy_search("hello", &["hello", "helo", "help"], 0.3);
    assert!(high_threshold.len() <= low_threshold.len());
}

#[test]
fn test_be_case_insensitive() {
    let result = umt_fuzzy_search("Hello", &["HELLO", "hello", "Hello"], 0.6);
    assert_eq!(result.len(), 3);
    assert!(result.iter().all(|r| r.score == 1.0));
}

#[test]
fn test_handle_empty_query() {
    let result = umt_fuzzy_search("", &["hello", "world"], 0.6);
    assert!(result.is_empty());
}

#[test]
fn test_handle_empty_items_array() {
    let result: Vec<FuzzyMatch> = umt_fuzzy_search("hello", &[], 0.6);
    assert!(result.is_empty());
}

#[test]
fn test_filter_out_items_below_threshold() {
    let result = umt_fuzzy_search("hello", &["world", "xyz"], 0.8);
    assert!(result.is_empty());
}

#[test]
fn test_handle_single_character_matches() {
    let result = umt_fuzzy_search("a", &["a", "b", "ab"], 0.5);
    assert!(result.iter().any(|m| m.item == "a" && m.score == 1.0));
}

#[test]
fn test_handle_special_characters() {
    let result = umt_fuzzy_search("hello!", &["hello!", "hello"], 0.6);
    assert_eq!(result[0].item, "hello!");
    assert_eq!(result[0].score, 1.0);
}

#[test]
fn test_fuzzy_search_default() {
    let result = umt_fuzzy_search_default("hello", &["hello", "helo"]);
    assert!(!result.is_empty());
}

#[test]
fn test_distance_budget_is_floored_from_threshold() {
    let result = umt_fuzzy_search("abcdefghij", &["abcdefghiX", "abcdefghij"], 0.9);
    assert_eq!(
        result,
        vec![FuzzyMatch {
            item: "abcdefghij".to_string(),
            score: 1.0
        }]
    );
}

#[test]
fn test_length_difference_filter() {
    let result = umt_fuzzy_search("abc", &["", "a", "abcdefgh", "ABC"], 0.3);
    let items: Vec<&str> = result.iter().map(|entry| entry.item.as_str()).collect();
    assert_eq!(items, vec!["ABC", "abcdefgh", "a"]);
    assert_eq!(result[1].score, 0.375);
    assert_eq!(result[2].score, 0.33333333333333337);
}

#[test]
fn test_early_exit_and_final_distance_over_budget() {
    assert!(umt_fuzzy_search("abcdef", &["uvwxyz"], 0.5).is_empty());
    assert!(umt_fuzzy_search("abcd", &["abxy"], 0.75).is_empty());
}

#[test]
fn test_query_longer_than_item() {
    let result = umt_fuzzy_search("kitchen", &["kitten"], 0.5);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].score, 0.7142857142857143);
}

#[test]
fn test_sorted_by_score_descending() {
    let result = umt_fuzzy_search("kitten", &["sitting", "mitten", "kitchen"], 0.5);
    let items: Vec<&str> = result.iter().map(|entry| entry.item.as_str()).collect();
    assert_eq!(items, vec!["mitten", "kitchen", "sitting"]);
}

#[test]
fn test_nan_threshold_keeps_every_item() {
    let result = umt_fuzzy_search("ab", &["xy"], f64::NAN);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].score, 0.0);
}

#[test]
fn test_case_insensitive_exact_threshold() {
    let result = umt_fuzzy_search("HeLLo", &["hEllO"], 1.0);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].score, 1.0);
    assert!(umt_fuzzy_search("", &["hEllO"], 1.0).is_empty());
}
