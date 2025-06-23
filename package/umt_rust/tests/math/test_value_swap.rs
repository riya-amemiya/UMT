use umt_rust::math::umt_value_swap;

#[test]
fn test_value_swap() {
    assert_eq!(umt_value_swap(1.0, 2.0), vec![1.0, 2.0]);
    assert_eq!(umt_value_swap(-1.0, 2.0), vec![-1.0, 2.0]);
    assert_eq!(umt_value_swap(1.0, -2.0), vec![-2.0, 1.0]);
    assert_eq!(umt_value_swap(0.0, 2.0), vec![0.0, 2.0]);
    assert_eq!(umt_value_swap(1.0, 0.0), vec![0.0, 1.0]);
}
