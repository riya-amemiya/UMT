use umt_rust::math::umt_standard_deviation;

#[test]
fn test_standard_deviation_sequential() {
    let result = umt_standard_deviation(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert!((result - std::f64::consts::SQRT_2).abs() < 1e-10);
}

#[test]
fn test_standard_deviation_three() {
    let result = umt_standard_deviation(&[1.0, 2.0, 3.0]);
    assert!((result - 0.816496580927726).abs() < 1e-10);
}

#[test]
fn test_standard_deviation_non_sequential() {
    let result = umt_standard_deviation(&[10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0]);
    assert!((result - 4.898979485566356).abs() < 1e-10);
}

#[test]
fn test_standard_deviation_5_to_25() {
    let result = umt_standard_deviation(&[5.0, 10.0, 15.0, 20.0, 25.0]);
    assert!((result - 7.0710678118654755).abs() < 1e-10);
}

#[test]
fn test_standard_deviation_same_values() {
    let result = umt_standard_deviation(&[5.0, 5.0, 5.0, 5.0]);
    assert_eq!(result, 0.0);
}

#[test]
fn test_standard_deviation_single() {
    let result = umt_standard_deviation(&[42.0]);
    assert_eq!(result, 0.0);
}

#[test]
fn test_standard_deviation_empty() {
    assert!(umt_standard_deviation(&[]).is_nan());
}
