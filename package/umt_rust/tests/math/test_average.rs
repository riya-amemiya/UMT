use umt_rust::math::umt_average;

#[test]
fn test_average() {
    assert_eq!(umt_average(vec![1.0, 2.0, 3.0]), 2.0);
    assert_eq!(umt_average(vec![-1.0, -2.0, -3.0]), -2.0);
    assert_eq!(umt_average(vec![]), 0.0);
}
