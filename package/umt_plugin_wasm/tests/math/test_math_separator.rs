use umt_plugin_wasm::math_separator;

#[test]
fn test_math_separator() {
    assert_eq!(math_separator(123), vec![100, 23]);
    assert_eq!(math_separator(1234), vec![1000, 234]);
    assert_eq!(math_separator(12345), vec![10000, 2345]);
}