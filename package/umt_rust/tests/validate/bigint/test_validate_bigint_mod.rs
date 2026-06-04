use umt_rust::validate::bigint::*;

#[test]
fn test_validate_bigint_positive() {
    let result = umt_validate_bigint(42i128, None);
    assert!(result.validate);
    assert_eq!(result.type_value, 42i128);
}

#[test]
fn test_validate_bigint_negative() {
    let result = umt_validate_bigint(-7i128, None);
    assert!(result.validate);
    assert_eq!(result.type_value, -7i128);
}

#[test]
fn test_bigint_validator() {
    let validator = umt_bigint_validator(None);
    let result = validator(100i128);
    assert!(result.validate);
}
