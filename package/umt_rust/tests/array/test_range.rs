use umt_rust::array::{umt_range, umt_range_f64};

#[test]
fn test_umt_range() {
    assert_eq!(umt_range(1, Some(5), None), vec![1, 2, 3, 4]);
    assert_eq!(umt_range(-2, Some(3), None), vec![-2, -1, 0, 1, 2]);
    assert_eq!(umt_range(0, Some(0), None), Vec::<i32>::new());
}

#[test]
fn test_umt_range_no_end() {
    // When end is None, generates from 0 to start
    assert_eq!(umt_range(5, None, None), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_umt_range_with_step() {
    assert_eq!(umt_range(2, Some(10), Some(2)), vec![2, 4, 6, 8]);
    assert_eq!(umt_range(0, Some(10), Some(3)), vec![0, 3, 6, 9]);
}

#[test]
fn test_umt_range_negative_step() {
    assert_eq!(umt_range(5, Some(1), Some(-1)), vec![5, 4, 3, 2]);
    assert_eq!(umt_range(10, Some(0), Some(-2)), vec![10, 8, 6, 4, 2]);
}

#[test]
fn test_umt_range_zero_step() {
    assert_eq!(umt_range(1, Some(5), Some(0)), Vec::<i32>::new());
}

#[test]
fn test_umt_range_invalid_positive_step() {
    // Positive step but start >= end
    assert_eq!(umt_range(5, Some(1), Some(1)), Vec::<i32>::new());
    assert_eq!(umt_range(5, Some(5), Some(1)), Vec::<i32>::new());
}

#[test]
fn test_umt_range_invalid_negative_step() {
    // Negative step but start <= end
    assert_eq!(umt_range(1, Some(5), Some(-1)), Vec::<i32>::new());
    assert_eq!(umt_range(5, Some(5), Some(-1)), Vec::<i32>::new());
}

#[test]
fn test_umt_range_f64_basic() {
    assert_eq!(
        umt_range_f64(5.0, None, None),
        vec![0.0, 1.0, 2.0, 3.0, 4.0]
    );
    assert_eq!(
        umt_range_f64(1.0, Some(5.0), None),
        vec![1.0, 2.0, 3.0, 4.0]
    );
}

#[test]
fn test_umt_range_f64_with_step() {
    assert_eq!(
        umt_range_f64(0.0, Some(1.0), Some(0.2)),
        vec![0.0, 0.2, 0.4, 0.6, 0.8]
    );
}

#[test]
fn test_umt_range_f64_negative_step() {
    assert_eq!(
        umt_range_f64(5.0, Some(1.0), Some(-1.0)),
        vec![5.0, 4.0, 3.0, 2.0]
    );
}

#[test]
fn test_umt_range_f64_zero_step() {
    assert_eq!(umt_range_f64(1.0, Some(5.0), Some(0.0)), Vec::<f64>::new());
}

#[test]
fn test_umt_range_f64_invalid_positive_step() {
    assert_eq!(umt_range_f64(5.0, Some(1.0), Some(1.0)), Vec::<f64>::new());
    assert_eq!(umt_range_f64(5.0, Some(5.0), Some(1.0)), Vec::<f64>::new());
}

#[test]
fn test_umt_range_f64_invalid_negative_step() {
    assert_eq!(umt_range_f64(1.0, Some(5.0), Some(-1.0)), Vec::<f64>::new());
    assert_eq!(umt_range_f64(5.0, Some(5.0), Some(-1.0)), Vec::<f64>::new());
}
