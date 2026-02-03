use umt_rust::validate::{umt_is_browser, umt_is_bun, umt_is_node, umt_is_node_webkit};

#[test]
fn test_environment_checks() {
    // These should all return false in a native environment
    assert!(!umt_is_browser());
    assert!(!umt_is_bun());
    assert!(!umt_is_node());
    assert!(!umt_is_node_webkit());
}
