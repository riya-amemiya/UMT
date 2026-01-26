use umt_rust::string::{umt_random_string, umt_random_string_default};

#[test]
fn test_generate_random_string_with_default_characters_and_length() {
    let str_result = umt_random_string_default(8);
    assert_eq!(str_result.len(), 8);
    assert!(str_result.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_generate_random_string_using_custom_character_set() {
    let chars = "abc123";
    let str_result = umt_random_string(10, Some(chars));
    assert_eq!(str_result.len(), 10);
    assert!(str_result.chars().all(|c| chars.contains(c)));
}

#[test]
fn test_generate_random_string_with_specified_length() {
    let size = 20;
    let str_result = umt_random_string(size, None);
    assert_eq!(str_result.len(), size);
}

#[test]
fn test_generate_random_string_with_zero_size() {
    let str_result = umt_random_string(0, None);
    assert!(str_result.is_empty());
}

#[test]
fn test_generate_random_string_custom_chars_all_included() {
    let chars = "ABC";
    let str_result = umt_random_string(100, Some(chars));
    assert!(str_result.chars().all(|c| chars.contains(c)));
}
