use umt_rust::math::{umt_lcm, umt_lcm_multiple};

#[test]
fn test_umt_lcm() {
    assert_eq!(umt_lcm(12, 18), 36);
    assert_eq!(umt_lcm(5, 7), 35);
    assert_eq!(umt_lcm(10, 0), 0);
    assert_eq!(umt_lcm(0, 10), 0);
    assert_eq!(umt_lcm(-12, 18), 36);
    assert_eq!(umt_lcm(12, -18), 36);
    assert_eq!(umt_lcm(-12, -18), 36);
}

#[test]
fn test_umt_lcm_zero() {
    assert_eq!(umt_lcm(0, 0), 0);
}

#[test]
fn test_umt_lcm_multiple() {
    assert_eq!(umt_lcm_multiple(&[2, 3]), 6);
    assert_eq!(umt_lcm_multiple(&[2, 3, 4]), 12);
    assert_eq!(umt_lcm_multiple(&[4, 6, 8, 12]), 24);
    assert_eq!(umt_lcm_multiple(&[-2, 3, -5]), 30);
}

#[test]
fn test_umt_lcm_multiple_with_zero() {
    assert_eq!(umt_lcm_multiple(&[2, 3, 0]), 0);
    assert_eq!(umt_lcm_multiple(&[2, 0, 5]), 0);
}

#[test]
#[should_panic(expected = "At least two numbers are required to compute LCM")]
fn test_umt_lcm_multiple_too_few() {
    umt_lcm_multiple(&[5]);
}
