use umt_rust::math::umt_percentile;

#[test]
fn test_percentile_50() {
    assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 50.0).unwrap(), 3.0);
}

#[test]
fn test_percentile_25() {
    assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 25.0).unwrap(), 2.0);
}

#[test]
fn test_percentile_75() {
    assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 75.0).unwrap(), 4.0);
}

#[test]
fn test_percentile_0() {
    assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.0).unwrap(), 1.0);
}

#[test]
fn test_percentile_100() {
    assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 100.0).unwrap(), 5.0);
}

#[test]
fn test_percentile_single_element() {
    assert_eq!(umt_percentile(&[42.0], 50.0).unwrap(), 42.0);
    assert_eq!(umt_percentile(&[42.0], 0.0).unwrap(), 42.0);
    assert_eq!(umt_percentile(&[42.0], 100.0).unwrap(), 42.0);
}

#[test]
fn test_percentile_empty() {
    assert!(umt_percentile(&[], 50.0).unwrap().is_nan());
}

#[test]
fn test_percentile_invalid_negative() {
    assert!(umt_percentile(&[1.0, 2.0, 3.0], -1.0).is_err());
}

#[test]
fn test_percentile_invalid_over_100() {
    assert!(umt_percentile(&[1.0, 2.0, 3.0], 101.0).is_err());
}

#[test]
fn test_percentile_unsorted() {
    assert_eq!(umt_percentile(&[5.0, 1.0, 3.0, 2.0, 4.0], 50.0).unwrap(), 3.0);
}

#[test]
fn test_percentile_negative() {
    assert_eq!(umt_percentile(&[-5.0, -3.0, -1.0, 1.0, 3.0], 50.0).unwrap(), -1.0);
}

#[test]
fn test_percentile_duplicates() {
    assert_eq!(umt_percentile(&[1.0, 1.0, 2.0, 2.0, 3.0, 3.0], 50.0).unwrap(), 2.0);
}
