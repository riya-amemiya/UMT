use umt_rust::ua::{Browser, umt_extract_browser_from_user_agent};

#[test]
fn test_should_detect_edge_browser() {
    let edge_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    let legacy_edge_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edge/89.0.774.57";
    assert_eq!(umt_extract_browser_from_user_agent(edge_ua), Browser::Edge);
    assert_eq!(
        umt_extract_browser_from_user_agent(legacy_edge_ua),
        Browser::Edge
    );
}

#[test]
fn test_should_detect_desktop_chrome_browser() {
    let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    assert_eq!(
        umt_extract_browser_from_user_agent(chrome_ua),
        Browser::Chrome
    );
}

#[test]
fn test_should_detect_ios_chrome_browser() {
    let chrome_ios_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/89.0.4389.90 Mobile/15E148 Safari/604.1";
    let chrome_ipad_ua = "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/89.0.4389.90 Mobile/15E148 Safari/604.1";
    assert_eq!(
        umt_extract_browser_from_user_agent(chrome_ios_ua),
        Browser::Chrome
    );
    assert_eq!(
        umt_extract_browser_from_user_agent(chrome_ipad_ua),
        Browser::Chrome
    );
}

#[test]
fn test_should_detect_firefox_browser() {
    let firefox_ua =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:88.0) Gecko/20100101 Firefox/88.0";
    let firefox_ios_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/33.0 Mobile/15E148 Safari/605.1.15";
    assert_eq!(
        umt_extract_browser_from_user_agent(firefox_ua),
        Browser::Firefox
    );
    assert_eq!(
        umt_extract_browser_from_user_agent(firefox_ios_ua),
        Browser::Firefox
    );
}

#[test]
fn test_should_detect_safari_browser() {
    let safari_ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    let mobile_safari_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    assert_eq!(
        umt_extract_browser_from_user_agent(safari_ua),
        Browser::Safari
    );
    assert_eq!(
        umt_extract_browser_from_user_agent(mobile_safari_ua),
        Browser::Safari
    );
}

#[test]
fn test_should_detect_internet_explorer() {
    let ie11_ua = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
    let ie10_ua = "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; Trident/6.0)";
    assert_eq!(umt_extract_browser_from_user_agent(ie11_ua), Browser::Ie);
    assert_eq!(umt_extract_browser_from_user_agent(ie10_ua), Browser::Ie);
}

#[test]
fn test_should_return_other_for_unknown_browsers() {
    let opera_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 OPR/75.0.3969.149";
    let unknown_ua = "Unknown Browser";
    let empty_ua = "";
    assert_eq!(
        umt_extract_browser_from_user_agent(opera_ua),
        Browser::Other
    );
    assert_eq!(
        umt_extract_browser_from_user_agent(unknown_ua),
        Browser::Other
    );
    assert_eq!(
        umt_extract_browser_from_user_agent(empty_ua),
        Browser::Other
    );
}

#[test]
fn test_browser_as_str_method() {
    assert_eq!(Browser::Edge.as_str(), "edge");
    assert_eq!(Browser::Chrome.as_str(), "chrome");
    assert_eq!(Browser::Firefox.as_str(), "firefox");
    assert_eq!(Browser::Safari.as_str(), "safari");
    assert_eq!(Browser::Ie.as_str(), "ie");
    assert_eq!(Browser::Other.as_str(), "other");
}
