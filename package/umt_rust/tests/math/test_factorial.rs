use umt_rust::math::umt_factorial;

#[test]
fn test_factorial() {
    assert_eq!(umt_factorial(0), 1);
    assert_eq!(umt_factorial(1), 1);
    assert_eq!(umt_factorial(2), 2);
    assert_eq!(umt_factorial(3), 6);
}
