//! Tests for is_browser function

use umt_rust::validate::umt_is_browser;

#[test]
fn test_is_browser_returns_false_on_native_target() {
    assert!(!umt_is_browser());
}

#[test]
fn test_is_browser_matches_target_configuration() {
    let expected = cfg!(all(target_arch = "wasm32", not(target_os = "wasi")));
    assert_eq!(umt_is_browser(), expected);
}
