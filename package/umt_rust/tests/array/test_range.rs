use umt_rust::array::umt_range;

#[test]
fn test_umt_range() {
    assert_eq!(umt_range(1, 5), vec![1, 2, 3, 4]);
    assert_eq!(umt_range(-2, 3), vec![-2, -1, 0, 1, 2]);
    assert_eq!(umt_range(0, 0), vec![]);
}
