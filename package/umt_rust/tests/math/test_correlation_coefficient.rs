use umt_rust::math::umt_correlation_coefficient;

#[test]
fn test_perfect_positive_correlation() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 4.0, 6.0, 8.0, 10.0]);
    assert!((r - 1.0).abs() < 1e-10);
}

#[test]
fn test_perfect_negative_correlation() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[5.0, 4.0, 3.0, 2.0, 1.0]);
    assert!((r - (-1.0)).abs() < 1e-10);
}

#[test]
fn test_no_variance_returns_nan() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[1.0, 1.0, 1.0, 1.0, 1.0]);
    assert!(r.is_nan());
}

#[test]
fn test_different_lengths_returns_nan() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0], &[1.0, 2.0]);
    assert!(r.is_nan());
}

#[test]
fn test_empty_arrays_returns_nan() {
    let r = umt_correlation_coefficient(&[], &[]);
    assert!(r.is_nan());
}

#[test]
fn test_single_element_returns_nan() {
    let r = umt_correlation_coefficient(&[1.0], &[2.0]);
    assert!(r.is_nan());
}

#[test]
fn test_partial_correlation() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 3.0, 5.0, 4.0, 6.0]);
    assert!(r > 0.0);
    assert!(r < 1.0);
}

#[test]
fn test_negative_numbers() {
    let r = umt_correlation_coefficient(&[-1.0, -2.0, -3.0], &[-2.0, -4.0, -6.0]);
    assert!((r - 1.0).abs() < 1e-10);
}

#[test]
fn test_decimal_numbers() {
    let r = umt_correlation_coefficient(&[1.5, 2.5, 3.5], &[3.0, 5.0, 7.0]);
    assert!((r - 1.0).abs() < 1e-10);
}

#[test]
fn test_first_array_no_variance() {
    let r = umt_correlation_coefficient(&[1.0, 1.0, 1.0], &[1.0, 2.0, 3.0]);
    assert!(r.is_nan());
}

#[test]
fn test_second_array_no_variance() {
    let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0], &[2.0, 2.0, 2.0]);
    assert!(r.is_nan());
}
