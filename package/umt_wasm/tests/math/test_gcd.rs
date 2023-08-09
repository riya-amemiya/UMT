use umtn_wasm::gcd;

#[test]
fn test_gcd() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(0, 1), 1);
    assert_eq!(gcd(1, 0), 1);
}
