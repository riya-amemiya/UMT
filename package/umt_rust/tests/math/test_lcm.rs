use umt_rust::math::umt_lcm;

#[test]
fn test_umt_lcm() {
    assert_eq!(umt_lcm(12, 18), 36);
    assert_eq!(umt_lcm(5, 7), 35);
    assert_eq!(umt_lcm(10, 0), 0);
    assert_eq!(umt_lcm(0, 10), 0);
    assert_eq!(umt_lcm(-12, 18), -36);
    assert_eq!(umt_lcm(12, -18), -36);
    assert_eq!(umt_lcm(-12, -18), 36);
}

#[test]
fn test_umt_lcm_zero() {
    assert_eq!(umt_lcm(0, 0), 0);
}
