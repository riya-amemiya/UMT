use umt_plugin_wasm::deviation_value;

#[test]
fn test_deviation_value() {
    assert_eq!(deviation_value(100.0, 50.0, 10.0), 100.0);
    assert_eq!(deviation_value(100.0, 50.0, 20.0), 75.0);
}