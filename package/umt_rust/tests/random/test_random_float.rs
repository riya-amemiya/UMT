use umt_rust::random::umt_random_float;

#[test]
fn test_returns_values_within_the_half_open_interval() {
    for _ in 0..100 {
        let value = umt_random_float(5.0, 10.0);
        assert!(value >= 5.0);
        assert!(value < 10.0);
    }
}

#[test]
fn test_returns_the_lower_bound_when_min_equals_max() {
    assert_eq!(umt_random_float(7.0, 7.0), 7.0);
}

#[test]
fn test_supports_negative_ranges() {
    for _ in 0..50 {
        let value = umt_random_float(-5.0, -1.0);
        assert!(value >= -5.0);
        assert!(value < -1.0);
    }
}
