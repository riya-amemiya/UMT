use std::collections::HashMap;
use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::*;

#[test]
fn test_validate_object_no_validators() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "test".to_string());
    let result = umt_validate_object(&obj, None, None);
    assert!(result.validate);
}

#[test]
fn test_optional_none() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(validator(&None));
}

#[test]
fn test_optional_some_valid() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(validator(&Some(5)));
}

#[test]
fn test_optional_some_invalid() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(!validator(&Some(-1)));
}

#[test]
fn test_validate_optional_none() {
    let result = umt_validate_optional(&None::<i32>, |v| ValidateCoreReturnType {
        validate: *v > 0,
        message: String::new(),
        type_value: *v,
    });
    assert!(result.validate);
}
