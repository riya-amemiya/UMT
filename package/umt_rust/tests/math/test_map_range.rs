use umt_rust::math::umt_map_range;

#[test]
fn test_map_range_between_ranges() {
    assert_eq!(umt_map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
    assert_eq!(umt_map_range(0.5, 0.0, 1.0, 10.0, 20.0), 15.0);
    assert_eq!(umt_map_range(25.0, 0.0, 100.0, -1.0, 1.0), -0.5);
}

#[test]
fn test_map_range_endpoints_and_inverted_output() {
    assert_eq!(umt_map_range(0.0, 0.0, 10.0, 100.0, 200.0), 100.0);
    assert_eq!(umt_map_range(10.0, 0.0, 10.0, 100.0, 200.0), 200.0);
    assert_eq!(umt_map_range(2.5, 0.0, 10.0, 100.0, 0.0), 75.0);
}

#[test]
fn test_map_range_no_clamping() {
    assert_eq!(umt_map_range(15.0, 0.0, 10.0, 0.0, 100.0), 150.0);
    assert_eq!(umt_map_range(-5.0, 0.0, 10.0, 0.0, 100.0), -50.0);
}

#[test]
fn test_map_range_empty_input_range() {
    assert_eq!(umt_map_range(7.0, 5.0, 5.0, 10.0, 20.0), 10.0);
}
