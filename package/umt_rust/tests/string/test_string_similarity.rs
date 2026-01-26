use umt_rust::string::umt_string_similarity;

#[test]
fn test_return_1_for_identical_strings() {
    assert_eq!(umt_string_similarity("hello", "hello"), 1.0);
    assert_eq!(umt_string_similarity("world", "world"), 1.0);
}

#[test]
fn test_return_0_for_empty_strings() {
    assert_eq!(umt_string_similarity("", "hello"), 0.0);
    assert_eq!(umt_string_similarity("hello", ""), 0.0);
    assert_eq!(umt_string_similarity("", ""), 1.0); // both empty are identical
}

#[test]
fn test_calculate_similarity_for_similar_strings() {
    let similarity1 = umt_string_similarity("cat", "bat");
    assert!((similarity1 - 0.667).abs() < 0.01); // 1 - (1/3)

    let similarity2 = umt_string_similarity("kitten", "sitting");
    assert!((similarity2 - 0.571).abs() < 0.01); // 1 - (3/7)
}

#[test]
fn test_return_0_for_completely_different_strings() {
    let similarity = umt_string_similarity("abc", "xyz");
    assert_eq!(similarity, 0.0); // 1 - (3/3)
}

#[test]
fn test_handle_strings_of_different_lengths() {
    let similarity1 = umt_string_similarity("cat", "cats");
    assert_eq!(similarity1, 0.75); // 1 - (1/4)

    let similarity2 = umt_string_similarity("hello", "helo");
    assert_eq!(similarity2, 0.8); // 1 - (1/5)
}

#[test]
fn test_be_case_sensitive() {
    let similarity = umt_string_similarity("Hello", "hello");
    assert_eq!(similarity, 0.8); // 1 - (1/5)
}

#[test]
fn test_handle_unicode_characters() {
    let similarity1 = umt_string_similarity("café", "cafe");
    assert_eq!(similarity1, 0.75); // 1 - (1/4)

    // Note: emoji handling may differ from JS due to grapheme clusters
    // Testing basic unicode behavior
    let similarity2 = umt_string_similarity("こんにちは", "こんばんは");
    // 5 characters, 2 differences, similarity = 1 - (2/5) = 0.6
    assert!((similarity2 - 0.6).abs() < 0.01);
}

#[test]
fn test_return_values_between_0_and_1() {
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
