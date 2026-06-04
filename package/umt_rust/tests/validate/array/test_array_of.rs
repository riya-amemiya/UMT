//! Tests for array_of function

use std::cell::RefCell;
use std::rc::Rc;

use umt_rust::validate::array::umt_array_of;
use umt_rust::validate::core::ValidateCoreReturnType;

fn non_negative(value: &i32) -> ValidateCoreReturnType<i32> {
    ValidateCoreReturnType {
        validate: *value >= 0,
        message: if *value >= 0 {
            String::new()
        } else {
            "negative".to_string()
        },
        type_value: *value,
    }
}

#[test]
fn test_array_of_validates_empty_array() {
    let validator = umt_array_of(non_negative);
    let arr: Vec<i32> = vec![];
    let result = validator(&arr);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_array_of_validates_array_of_valid_elements() {
    let validator = umt_array_of(non_negative);
    let result = validator(&vec![0, 1, 2]);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_array_of_fails_when_an_element_does_not_match() {
    let validator = umt_array_of(non_negative);
    let result = validator(&vec![0, -1]);
    assert!(!result.validate);
}

#[test]
fn test_array_of_propagates_the_element_validation_message() {
    let validator = umt_array_of(non_negative);
    let result = validator(&vec![1, -1, -2]);
    assert!(!result.validate);
    assert_eq!(result.message, "negative");
}

#[test]
fn test_array_of_stops_at_the_first_failing_element() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let validator = umt_array_of(move |value: &i32| {
        recorded.borrow_mut().push(*value);
        non_negative(value)
    });
    let result = validator(&vec![1, -1, -2]);
    assert!(!result.validate);
    assert_eq!(result.message, "negative");
    assert_eq!(*calls.borrow(), vec![1, -1]);
}

#[test]
fn test_array_of_preserves_the_original_array_in_the_type_field() {
    let validator = umt_array_of(non_negative);
    let arr = vec![0, 1, 2];
    let result = validator(&arr);
    assert_eq!(result.type_value, arr);
}

#[test]
fn test_array_of_validates_arrays_of_strings() {
    let validator = umt_array_of(|value: &String| ValidateCoreReturnType {
        validate: !value.is_empty(),
        message: if value.is_empty() {
            "empty".to_string()
        } else {
            String::new()
        },
        type_value: value.clone(),
    });
    let result = validator(&vec!["a".to_string(), "b".to_string()]);
    assert!(result.validate);
    let bad = validator(&vec!["a".to_string(), String::new()]);
    assert!(!bad.validate);
    assert_eq!(bad.message, "empty");
}
