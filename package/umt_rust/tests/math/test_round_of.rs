use umt_rust::math::umt_round_of;


#[test]
fn test_round_of() {
    assert_eq!(umt_round_of(3.14159, 2), 3.14);
    assert_eq!(umt_round_of(3.145, 2), 3.15);
    assert_eq!(umt_round_of(0.0, 5), 0.0);
    assert_eq!(umt_round_of(-2.71828, 3), -2.718);
    assert_eq!(umt_round_of(123456.789, 1), 123456.8);
}
