use umtn_wasm::value_swap;

#[test]
fn test_value_swap() {
    assert_eq!(value_swap(1.0, 2.0), vec![2.0, 1.0]);
}