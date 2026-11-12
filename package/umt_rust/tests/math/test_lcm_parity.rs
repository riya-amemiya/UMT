use umt_rust::math::umt_lcm;

#[test]
fn test_lcm_negative_inputs_give_positive_result() {
    assert_eq!(umt_lcm(-4, 6), 12);
    assert_eq!(umt_lcm(4, -6), 12);
    assert_eq!(umt_lcm(-4, -6), 12);
}

#[test]
fn test_lcm_zero_inputs() {
    assert_eq!(umt_lcm(0, 3), 0);
    assert_eq!(umt_lcm(3, 0), 0);
    assert_eq!(umt_lcm(0, 0), 0);
}

#[test]
fn test_lcm_positive_inputs() {
    assert_eq!(umt_lcm(2, 3), 6);
    assert_eq!(umt_lcm(4, 6), 12);
}
