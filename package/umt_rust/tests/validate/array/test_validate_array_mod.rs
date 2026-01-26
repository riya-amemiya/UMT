use umt_rust::validate::array::*;

#[test]
fn test_validate_array_no_validator() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_with_validator() {
    let arr = vec![2, 4, 6];
    let result = umt_validate_array(&arr, Some(|x: &i32| x % 2 == 0), None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_fails() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, Some(|x: &i32| x % 2 == 0), Some("must be even"));
    assert!(!result.validate);
    assert_eq!(result.message, "must be even");
}
