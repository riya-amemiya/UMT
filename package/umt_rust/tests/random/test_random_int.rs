use umt_rust::random::umt_random_int;

#[test]
fn test_returns_integers_within_the_closed_interval() {
    for _ in 0..200 {
        let value = umt_random_int(1.0, 6.0);
        assert_eq!(value, value.floor());
        assert!(value >= 1.0);
        assert!(value <= 6.0);
    }
}

#[test]
fn test_returns_the_bound_when_min_equals_max() {
    assert_eq!(umt_random_int(3.0, 3.0), 3.0);
}

#[test]
fn test_supports_negative_ranges() {
    for _ in 0..50 {
        let value = umt_random_int(-3.0, 3.0);
        assert!(value >= -3.0);
        assert!(value <= 3.0);
    }
}

#[test]
fn test_ceils_the_lower_bound_and_floors_the_upper_bound() {
    for _ in 0..50 {
        let value = umt_random_int(0.4, 5.7);
        assert!(value >= 1.0);
        assert!(value <= 5.0);
    }
}
