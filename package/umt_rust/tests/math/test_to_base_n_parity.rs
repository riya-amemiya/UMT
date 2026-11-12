use umt_rust::math::umt_to_base_n;

#[test]
fn test_to_base_n_minimum_value() {
    assert_eq!(umt_to_base_n(i64::MIN, 2), format!("-1{}", "0".repeat(63)));
    assert_eq!(umt_to_base_n(i64::MIN, 16), "-8000000000000000");
}

#[test]
fn test_to_base_n_negative_value() {
    assert_eq!(umt_to_base_n(-255, 36), "-73");
    assert_eq!(umt_to_base_n(-10, 2), "-1010");
}

#[test]
fn test_to_base_n_positive_value() {
    assert_eq!(umt_to_base_n(100, 7), "202");
    assert_eq!(umt_to_base_n(i64::MAX, 36), "1y2p0ij32e8e7");
}

#[test]
fn test_to_base_n_zero_and_invalid_radix() {
    assert_eq!(umt_to_base_n(0, 2), "0");
    assert_eq!(umt_to_base_n(10, 1), "Invalid radix");
    assert_eq!(umt_to_base_n(10, 37), "Invalid radix");
}
