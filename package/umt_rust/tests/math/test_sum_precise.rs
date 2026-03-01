use umt_rust::math::umt_sum_precise;

#[test]
fn test_sum_simple_integers() {
    assert_eq!(umt_sum_precise(&[1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
}

#[test]
fn test_handle_floating_point_precision() {
    let result = umt_sum_precise(&[0.1, 0.2, 0.3]);
    assert!((result - 0.6).abs() < 1e-15);
}

#[test]
fn test_handle_large_and_small_number_cancellation() {
    assert_eq!(umt_sum_precise(&[1e20, 1.0, -1e20]), 1.0);
}

#[test]
fn test_return_zero_for_empty_array() {
    assert_eq!(umt_sum_precise(&Vec::<f64>::new()), 0.0);
}

#[test]
fn test_return_single_element() {
    assert_eq!(umt_sum_precise(&[42.0]), 42.0);
}

#[test]
fn test_handle_all_negative_numbers() {
    assert_eq!(umt_sum_precise(&[-1.0, -2.0, -3.0]), -6.0);
}

#[test]
fn test_handle_mixed_positive_and_negative_numbers() {
    assert_eq!(umt_sum_precise(&[10.0, -3.0, 5.0, -2.0]), 10.0);
}

#[test]
fn test_handle_many_small_floating_point_values() {
    let arr = vec![0.001; 1000];
    let result = umt_sum_precise(&arr);
    assert!((result - 1.0).abs() < 1e-10);
}
