use umt_rust::math::umt_clamp;

#[test]
fn test_should_return_the_value_when_within_range() {
    assert_eq!(umt_clamp(5.0, 0.0, 10.0), 5.0);
}

#[test]
fn test_should_return_the_min_when_value_is_below_range() {
    assert_eq!(umt_clamp(-3.0, 0.0, 10.0), 0.0);
}

#[test]
fn test_should_return_the_max_when_value_is_above_range() {
    assert_eq!(umt_clamp(15.0, 0.0, 10.0), 10.0);
}

#[test]
fn test_should_return_min_when_value_equals_min() {
    assert_eq!(umt_clamp(0.0, 0.0, 10.0), 0.0);
}

#[test]
fn test_should_return_max_when_value_equals_max() {
    assert_eq!(umt_clamp(10.0, 0.0, 10.0), 10.0);
}

#[test]
fn test_should_handle_negative_ranges() {
    assert_eq!(umt_clamp(-5.0, -10.0, -1.0), -5.0);
    assert_eq!(umt_clamp(0.0, -10.0, -1.0), -1.0);
    assert_eq!(umt_clamp(-20.0, -10.0, -1.0), -10.0);
}

#[test]
fn test_should_handle_floating_point_values() {
    assert_eq!(umt_clamp(0.5, 0.0, 1.0), 0.5);
    assert_eq!(umt_clamp(1.5, 0.0, 1.0), 1.0);
    assert_eq!(umt_clamp(-0.5, 0.0, 1.0), 0.0);
}

#[test]
fn test_should_handle_min_equal_to_max() {
    assert_eq!(umt_clamp(5.0, 3.0, 3.0), 3.0);
    assert_eq!(umt_clamp(1.0, 3.0, 3.0), 3.0);
}
