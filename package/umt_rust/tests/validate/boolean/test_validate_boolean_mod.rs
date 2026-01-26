use umt_rust::validate::boolean::*;

#[test]
fn test_validate_boolean_true() {
    let result = umt_validate_boolean(true, None);
    assert!(result.validate);
    assert!(result.type_value);
}

#[test]
fn test_validate_boolean_false() {
    let result = umt_validate_boolean(false, None);
    assert!(result.validate);
    assert!(!result.type_value);
}

#[test]
fn test_boolean_validator() {
    let validator = umt_boolean_validator(None);
    let result = validator(true);
    assert!(result.validate);
}
