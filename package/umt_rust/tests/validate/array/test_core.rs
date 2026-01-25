//! Tests for array core validation

use umt_rust::validate::array::umt_validate_array;

#[test]
fn test_validate_empty_array() {
    let arr: Vec<i32> = vec![];
    let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_of_mixed_values_without_type_validation() {
    // In Rust, arrays are typed, so we test with a generic type
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_of_strings() {
    let arr = vec!["apple", "banana", "cherry"];
    let is_string_validator = |_: &&str| true;
    let result = umt_validate_array(&arr, Some(is_string_validator), None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_of_numbers_with_range_validation() {
    let arr = vec![1, 2, 3];
    let range_validator = |x: &i32| *x >= 0 && *x <= 10;
    let result = umt_validate_array(&arr, Some(range_validator), None);
    assert!(result.validate);

    let arr_invalid = vec![1, 2, 3, 11];
    let result_invalid = umt_validate_array(&arr_invalid, Some(range_validator), None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_validate_array_returns_custom_message_on_failure() {
    let arr = vec![1, 2, 3];
    let always_fail = |_: &i32| false;
    let custom_message = "Array validation failed";
    let result = umt_validate_array(&arr, Some(always_fail), Some(custom_message));
    assert!(!result.validate);
    assert_eq!(result.message, custom_message);
}

#[test]
fn test_validate_array_returns_empty_message_when_no_custom_message() {
    let arr = vec![1, 2, 3];
    let always_fail = |_: &i32| false;
    let result = umt_validate_array(&arr, Some(always_fail), None);
    assert!(!result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_validate_array_preserves_original_array() {
    let test_array = vec![1, 2, 3];
    let result = umt_validate_array(&test_array, None::<fn(&i32) -> bool>, None);
    assert_eq!(result.type_value, test_array);
}

#[test]
fn test_validate_array_with_even_number_validator() {
    let arr = vec![2, 4, 6];
    let even_validator = |x: &i32| x % 2 == 0;
    let result = umt_validate_array(&arr, Some(even_validator), None);
    assert!(result.validate);

    let arr_with_odd = vec![2, 3, 6];
    let result_invalid = umt_validate_array(&arr_with_odd, Some(even_validator), None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_validate_array_with_min_length_strings() {
    let arr = vec!["abc", "defg", "hijkl"];
    let min_length_validator = |s: &&str| s.len() >= 3;
    let result = umt_validate_array(&arr, Some(min_length_validator), None);
    assert!(result.validate);

    let arr_with_short = vec!["ok", "good", "toolong"];
    let min_length_3_validator = |s: &&str| s.len() >= 4;
    let result_invalid = umt_validate_array(
        &arr_with_short,
        Some(min_length_3_validator),
        Some("String too short"),
    );
    assert!(!result_invalid.validate);
}
