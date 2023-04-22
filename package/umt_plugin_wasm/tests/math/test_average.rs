use umt_plugin_wasm::average;

#[test]
fn test_average() {
    assert_eq!(average(vec![1.0, 2.0, 3.0]), 2.0);
}
