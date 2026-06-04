//! Tests for is_node_webkit function

use umt_rust::validate::umt_is_node_webkit;

#[test]
fn test_is_node_webkit_returns_false_on_native_target() {
    assert!(!umt_is_node_webkit());
}

#[test]
fn test_is_node_webkit_matches_target_configuration() {
    let expected = cfg!(all(target_arch = "wasm32", not(target_os = "wasi")))
        && cfg!(not(target_arch = "wasm32"));
    assert_eq!(umt_is_node_webkit(), expected);
}
