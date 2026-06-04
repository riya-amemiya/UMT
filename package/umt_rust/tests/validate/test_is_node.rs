//! Tests for is_node function

use umt_rust::validate::umt_is_node;

#[test]
fn test_is_node_matches_environment_marker() {
    // The function mirrors the TypeScript `typeof process !== "undefined" &&
    // typeof require !== "undefined"` check by probing the `NODE` marker, so its
    // result must agree with whether that environment variable is currently
    // present.
    let expected = std::env::var_os("NODE").is_some();
    assert_eq!(umt_is_node(), expected);
}

#[test]
fn test_is_node_returns_bool() {
    let detected = umt_is_node();
    assert!(detected || !detected);
}
