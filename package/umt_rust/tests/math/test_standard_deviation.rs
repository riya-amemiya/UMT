use umt_rust::math::{umt_standard_deviation, umt_standard_deviation_with_sample};

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
    assert_eq!(umt_standard_deviation(&[]), 0.0);
}

#[test]
fn test_standard_deviation_with_sample_population() {
    assert_eq!(
        umt_standard_deviation_with_sample(&[1.0, 2.0, 3.0], false),
        umt_standard_deviation(&[1.0, 2.0, 3.0])
    );
}

#[test]
fn test_standard_deviation_with_sample_sample() {
    assert_eq!(
        umt_standard_deviation_with_sample(&[1.0, 2.0, 3.0], true),
        1.0
    );
    assert_eq!(
        umt_standard_deviation_with_sample(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0], true),
        (32.0_f64 / 7.0).sqrt()
    );
    assert_eq!(umt_standard_deviation_with_sample(&[], true), 0.0);
    assert!(umt_standard_deviation_with_sample(&[42.0], true).is_nan());
}
