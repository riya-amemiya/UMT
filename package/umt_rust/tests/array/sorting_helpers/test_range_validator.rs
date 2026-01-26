use umt_rust::array::sorting_helpers::*;

#[test]
fn test_validate_range_empty_array() {
    let result = validate_range(0, 0, 0);
    assert_eq!(result.start_index, 0);
    assert_eq!(result.end_index, 0);
    assert!(!result.should_sort);
}

#[test]
fn test_validate_range_normal() {
    let result = validate_range(10, 2, 7);
    assert_eq!(result.start_index, 2);
    assert_eq!(result.end_index, 7);
    assert!(result.should_sort);
}

#[test]
fn test_validate_range_exceeds_bounds() {
    let result = validate_range(5, 0, 10);
    assert_eq!(result.start_index, 0);
    assert_eq!(result.end_index, 4);
    assert!(result.should_sort);
}

#[test]
fn test_validate_range_start_exceeds_bounds() {
    let result = validate_range(5, 10, 15);
    assert_eq!(result.start_index, 4);
    assert_eq!(result.end_index, 4);
    assert!(result.should_sort);
}

#[test]
fn test_validate_range_single_element() {
    let result = validate_range(1, 0, 0);
    assert_eq!(result.start_index, 0);
    assert_eq!(result.end_index, 0);
    assert!(result.should_sort);
}
