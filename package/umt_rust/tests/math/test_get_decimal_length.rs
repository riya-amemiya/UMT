use umt_rust::math::umt_get_decimal_length;



#[test]
fn test_get_decimal_length() {
    assert_eq!(umt_get_decimal_length(0.0), 0);
    assert_eq!(umt_get_decimal_length(1.0), 0);
    assert_eq!(umt_get_decimal_length(1.1), 1);
    assert_eq!(umt_get_decimal_length(1.11), 2);
}
