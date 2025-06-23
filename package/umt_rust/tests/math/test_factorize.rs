use umt_rust::math::umt_factorize;

#[test]
fn test_factorize() {
    assert_eq!(umt_factorize(0), vec![]);
    assert_eq!(umt_factorize(1), vec![1]);
    assert_eq!(umt_factorize(2), vec![2]);
    assert_eq!(umt_factorize(4), vec![2, 2]);
    assert_eq!(umt_factorize(10), vec![2, 5]);
    assert_eq!(umt_factorize(7), vec![7]);
    assert_eq!(umt_factorize(30), vec![2, 3, 5]);
    assert_eq!(umt_factorize(100), vec![2, 2, 5, 5]);
}
