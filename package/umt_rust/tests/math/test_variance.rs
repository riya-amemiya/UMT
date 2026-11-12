use umt_rust::math::umt_variance;

#[test]
fn test_variance_population() {
    assert_eq!(umt_variance(&[1.0, 2.0, 3.0], false), 0.6666666666666666);
    assert_eq!(umt_variance(&[1.0, 2.0, 3.0, 4.0, 5.0], false), 2.0);
    assert_eq!(
        umt_variance(&[10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], false),
        24.0
    );
}

#[test]
fn test_variance_sample() {
    assert_eq!(umt_variance(&[1.0, 2.0, 3.0], true), 1.0);
    assert_eq!(umt_variance(&[1.0, 2.0, 3.0, 4.0, 5.0], true), 2.5);
    assert_eq!(
        umt_variance(&[10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], true),
        27.428571428571427
    );
}

#[test]
fn test_variance_identical_values() {
    assert_eq!(umt_variance(&[5.0, 5.0, 5.0, 5.0], false), 0.0);
    assert_eq!(umt_variance(&[5.0, 5.0, 5.0, 5.0], true), 0.0);
}

#[test]
fn test_variance_single_value() {
    assert_eq!(umt_variance(&[42.0], false), 0.0);
    assert!(umt_variance(&[42.0], true).is_nan());
}

#[test]
fn test_variance_empty() {
    assert_eq!(umt_variance(&[], false), 0.0);
    assert_eq!(umt_variance(&[], true), 0.0);
}
