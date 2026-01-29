use umt_rust::string::umt_random_string_initialization;

#[test]
fn test_random_string_initialization_default() {
    let random_func = umt_random_string_initialization(None);
    let result = random_func(10);
    assert_eq!(result.len(), 10);
}

#[test]
fn test_random_string_initialization_custom() {
    let custom_char_set = "ABC123";
    let random_func = umt_random_string_initialization(Some(custom_char_set));
    let result = random_func(5);
    assert_eq!(result.len(), 5);
    assert!(result.chars().all(|c| custom_char_set.contains(c)));
}
