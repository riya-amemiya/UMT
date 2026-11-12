use umt_rust::ua::{Browser, umt_extract_browser_from_user_agent};

#[test]
fn test_browser_parity_case_folding_is_ascii_only() {
    assert_eq!(
        umt_extract_browser_from_user_agent("\u{017F}afari"),
        Browser::Other
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("\u{212A}HTML Chrome"),
        Browser::Chrome
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("f\u{0131}refox"),
        Browser::Other
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("ms\u{0130}e"),
        Browser::Other
    );
}

#[test]
fn test_browser_parity_ascii_case_is_ignored_for_every_browser() {
    assert_eq!(umt_extract_browser_from_user_agent("EDG/1"), Browser::Edge);
    assert_eq!(
        umt_extract_browser_from_user_agent("Edge/18"),
        Browser::Edge
    );
    assert_eq!(umt_extract_browser_from_user_agent("TRIDENT"), Browser::Ie);
    assert_eq!(
        umt_extract_browser_from_user_agent("FXIOS"),
        Browser::Firefox
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("OPR/1 CHROME"),
        Browser::Other
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("CRIOS"),
        Browser::Chrome
    );
    assert_eq!(
        umt_extract_browser_from_user_agent("SAFARI"),
        Browser::Safari
    );
    assert_eq!(umt_extract_browser_from_user_agent(""), Browser::Other);
}
