use umt_rust::math::umt_in_range;

#[test]
fn test_in_range_two_args() {
    // Porting "should return true when value is within [0, end)"
    assert!(umt_in_range(3.0, 0.0, 5.0));

    // Porting "should return false when value equals end (two-arg form)"
    assert!(!umt_in_range(5.0, 0.0, 5.0));

    // Porting "should return true when value equals 0 (two-arg form)"
    assert!(umt_in_range(0.0, 0.0, 5.0));

    // Porting "should return false for negative value (two-arg form)"
    assert!(!umt_in_range(-1.0, 0.0, 5.0));
}

#[test]
fn test_in_range_three_args() {
    // Porting "should return true when value is within [start, end)"
    assert!(umt_in_range(3.0, 2.0, 5.0));

    // Porting "should swap start and end if start > end"
    assert!(umt_in_range(3.0, 5.0, 2.0));

    // Porting "should return false when value equals the upper bound"
    assert!(!umt_in_range(5.0, 2.0, 5.0));

    // Porting "should return true when value equals the lower bound"
    assert!(umt_in_range(2.0, 2.0, 5.0));
}

#[test]
fn test_in_range_negative_ranges() {
    // Porting "should handle negative ranges"
    assert!(umt_in_range(-3.0, -5.0, -1.0));
    assert!(umt_in_range(-5.0, -5.0, -1.0));
    assert!(!umt_in_range(-1.0, -5.0, -1.0));
}

#[test]
fn test_in_range_floating_point() {
    // Porting "should handle floating point values"
    assert!(umt_in_range(0.5, 0.0, 1.0));
    assert!(!umt_in_range(1.5, 0.0, 1.0));
}

#[test]
fn test_in_range_negative_two_args() {
    // Porting "should handle negative ranges (two-arg form)"
    // inRange(-3, -5) -> start=-5, end=undefined -> lower=-5, upper=0
    assert!(umt_in_range(-3.0, 0.0, -5.0));
    assert!(!umt_in_range(0.0, 0.0, -5.0));
    assert!(umt_in_range(-5.0, 0.0, -5.0));
}

#[test]
fn test_in_range_nan() {
    // New test for parity with NaN handling
    assert!(!umt_in_range(3.0, f64::NAN, 5.0));
    assert!(!umt_in_range(3.0, 5.0, f64::NAN));
    assert!(!umt_in_range(f64::NAN, 2.0, 5.0));
}
