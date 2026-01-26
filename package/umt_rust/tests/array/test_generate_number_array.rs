use umt_rust::array::{umt_generate_number_array, umt_generate_number_array_i32};

#[test]
fn test_generate_basic() {
    assert_eq!(
        umt_generate_number_array(5, None, None, false),
        vec![0.0, 1.0, 2.0, 3.0, 4.0]
    );
}

#[test]
fn test_generate_with_min_max() {
    assert_eq!(
        umt_generate_number_array(5, Some(10.0), Some(14.0), false),
        vec![10.0, 11.0, 12.0, 13.0, 14.0]
    );
}

#[test]
fn test_generate_empty() {
    assert_eq!(
        umt_generate_number_array(0, None, None, false),
        Vec::<f64>::new()
    );
}

#[test]
fn test_generate_single_element() {
    assert_eq!(
        umt_generate_number_array(1, Some(10.0), Some(20.0), false),
        vec![10.0]
    );
}

#[test]
fn test_generate_negative_range() {
    let result = umt_generate_number_array(5, Some(-10.0), Some(-6.0), false);
    assert_eq!(result, vec![-10.0, -9.0, -8.0, -7.0, -6.0]);
}

#[test]
fn test_generate_equal_min_max() {
    let result = umt_generate_number_array(5, Some(10.0), Some(10.0), false);
    assert_eq!(result, vec![10.0, 10.0, 10.0, 10.0, 10.0]);
}

#[test]
fn test_generate_decimal_range() {
    let result = umt_generate_number_array(3, Some(1.5), Some(2.5), false);
    assert_eq!(result, vec![1.5, 2.0, 2.5]);
}

#[test]
fn test_generate_large_array() {
    let result = umt_generate_number_array(10_000, None, None, false);
    assert_eq!(result.len(), 10_000);
    assert_eq!(result[0], 0.0);
    assert_eq!(result[9999], 9999.0);
}

#[test]
fn test_generate_zero_range() {
    let result = umt_generate_number_array(5, Some(0.0), Some(4.0), false);
    assert_eq!(result, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_generate_random() {
    let result = umt_generate_number_array(5, Some(10.0), Some(14.0), true);
    assert_eq!(result.len(), 5);
    for &value in &result {
        assert!(value >= 10.0 && value <= 15.0); // floor can result in max+1
    }
}

#[test]
fn test_generate_random_single_element() {
    let result = umt_generate_number_array(1, Some(5.0), Some(15.0), true);
    assert_eq!(result.len(), 1);
    // Single element should be min value
    assert_eq!(result[0], 5.0);
}

// Tests for i32 version
#[test]
fn test_generate_i32_basic() {
    assert_eq!(
        umt_generate_number_array_i32(5, None, None, false),
        vec![0, 1, 2, 3, 4]
    );
}

#[test]
fn test_generate_i32_with_min_max() {
    assert_eq!(
        umt_generate_number_array_i32(5, Some(10), Some(14), false),
        vec![10, 11, 12, 13, 14]
    );
}

#[test]
fn test_generate_i32_empty() {
    assert_eq!(
        umt_generate_number_array_i32(0, None, None, false),
        Vec::<i32>::new()
    );
}

#[test]
fn test_generate_i32_single_element() {
    assert_eq!(
        umt_generate_number_array_i32(1, Some(10), Some(20), false),
        vec![10]
    );
}

#[test]
fn test_generate_i32_random() {
    let result = umt_generate_number_array_i32(5, Some(10), Some(14), true);
    assert_eq!(result.len(), 5);
    for &value in &result {
        assert!(value >= 10 && value <= 14);
    }
}

#[test]
fn test_generate_i32_random_single_element() {
    let result = umt_generate_number_array_i32(1, Some(5), Some(15), true);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 5);
}
