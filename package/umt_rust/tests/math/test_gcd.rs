use umt_rust::math::umt_gcd;



#[test]
fn test_gcd() {
    assert_eq!(umt_gcd(0, 0), 0);
    assert_eq!(umt_gcd(0, 1), 1);
    assert_eq!(umt_gcd(1, 0), 1);
}
