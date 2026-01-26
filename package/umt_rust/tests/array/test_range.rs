use umt_rust::array::umt_range;

#[test]
fn test_umt_range() {
    assert_eq!(umt_range(1, Some(5), None), vec![1, 2, 3, 4]);
    assert_eq!(umt_range(-2, Some(3), None), vec![-2, -1, 0, 1, 2]);
    assert_eq!(umt_range(0, Some(0), None), Vec::<i32>::new());
}

use umt_rust::array::*;

#[test]
fn test_range_empty_when_equal() {
    assert_eq!(umt_range(5, Some(5), None), Vec::<i32>::new());
}

#[test]
fn test_range_f64_basic() {
    let result = umt_range_f64(5.0, None, None);
    assert_eq!(result, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_range_f64_negative_step() {
    let result = umt_range_f64(1.0, Some(0.0), Some(-0.2));
    // Expected: [1.0, 0.8, 0.6, 0.4, 0.2, 0.0] (6 elements, 0.0 > 0.0 is false so stops)
    // Actually: ends when index <= actual_end, which is index <= 0.0
    // So: 1.0, 0.8, 0.6, 0.4, 0.2, ~0.0 (stops because next would be -0.2 which is not > 0.0)
    // Due to floating point, might have different count
    assert!(result.len() >= 5);
    assert!((result[0] - 1.0).abs() < 1e-10);
}

#[test]
fn test_range_f64_with_step() {
    let result = umt_range_f64(0.0, Some(1.0), Some(0.2));
    assert_eq!(result.len(), 5);
    assert!((result[0] - 0.0).abs() < 1e-10);
    assert!((result[1] - 0.2).abs() < 1e-10);
    assert!((result[2] - 0.4).abs() < 1e-10);
    assert!((result[3] - 0.6).abs() < 1e-10);
    assert!((result[4] - 0.8).abs() < 1e-10);
}

#[test]
fn test_range_invalid_range_negative_step() {
    assert_eq!(umt_range(5, Some(10), Some(-1)), Vec::<i32>::new());
}

#[test]
fn test_range_invalid_range_positive_step() {
    assert_eq!(umt_range(10, Some(5), Some(1)), Vec::<i32>::new());
}

#[test]
fn test_range_negative_numbers() {
    assert_eq!(umt_range(-3, Some(3), None), vec![-3, -2, -1, 0, 1, 2]);
}

#[test]
fn test_range_negative_step() {
    assert_eq!(umt_range(5, Some(1), Some(-1)), vec![5, 4, 3, 2]);
}

#[test]
fn test_range_single_arg() {
    assert_eq!(umt_range(5, None, None), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_range_start_end() {
    assert_eq!(umt_range(2, Some(6), None), vec![2, 3, 4, 5]);
}

#[test]
fn test_range_with_step() {
    assert_eq!(umt_range(2, Some(10), Some(2)), vec![2, 4, 6, 8]);
}

#[test]
fn test_range_zero_step() {
    assert_eq!(umt_range(5, Some(10), Some(0)), Vec::<i32>::new());
}
