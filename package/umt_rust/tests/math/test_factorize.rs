use umt_rust::math::umt_factorize;



#[test]
fn test_factorize(){
    assert_eq!(umt_factorize(1), vec![1]);
    assert_eq!(umt_factorize(2), vec![2]);
    assert_eq!(umt_factorize(10), vec![2, 5]);
}