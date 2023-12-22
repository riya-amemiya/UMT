use umtn_wasm::range;

#[test]
fn test_umt_range_function() {
    assert_eq!(range(1, 5), vec![1, 2, 3, 4]);
    assert_eq!(range(-2, 3), vec![-2, -1, 0, 1, 2]);
    assert_eq!(range(0, 0), vec![]);
}
