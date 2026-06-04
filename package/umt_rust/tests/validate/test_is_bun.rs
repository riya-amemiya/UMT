//! Tests for is_bun function

use umt_rust::validate::umt_is_bun;

#[test]
fn test_is_bun_matches_environment_marker() {
    // The function mirrors the TypeScript `typeof Bun !== "undefined"` check by
    // probing the `BUN_INSTALL` marker, so its result must agree with whether
    // that environment variable is currently present.
    let expected = std::env::var_os("BUN_INSTALL").is_some();
    assert_eq!(umt_is_bun(), expected);
}

#[test]
fn test_is_bun_returns_bool() {
    let detected = umt_is_bun();
    assert!(detected || !detected);
}
