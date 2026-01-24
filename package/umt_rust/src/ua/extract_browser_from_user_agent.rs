use regex::Regex;

/// Represents the detected browser type from a User-Agent string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Browser {
    Edge,
    Chrome,
    Firefox,
    Safari,
    Ie,
    Other,
}

impl Browser {
    /// Converts the Browser enum to its string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Browser::Edge => "edge",
            Browser::Chrome => "chrome",
            Browser::Firefox => "firefox",
            Browser::Safari => "safari",
            Browser::Ie => "ie",
            Browser::Other => "other",
        }
    }
}

impl std::fmt::Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Extracts browser information from a User-Agent string.
///
/// # Arguments
///
/// * `ua` - The User-Agent string to analyze.
///
/// # Returns
///
/// The detected browser type (Edge, Chrome, Firefox, Safari, IE, or Other).
///
/// # Examples
///
/// ```
/// use umt_rust::ua::umt_extract_browser_from_user_agent;
///
/// let browser = umt_extract_browser_from_user_agent("Mozilla/5.0 Chrome/89.0");
/// assert_eq!(browser.as_str(), "chrome");
/// ```
#[inline]
pub fn umt_extract_browser_from_user_agent(ua: &str) -> Browser {
    let ua_lower = ua.to_lowercase();

    // Edge detection (includes both "Edg" and "Edge")
    let edge_re = Regex::new(r"edg(e)?").unwrap();
    if edge_re.is_match(&ua_lower) {
        return Browser::Edge;
    }

    // Internet Explorer detection
    let ie_re = Regex::new(r"msie|trident").unwrap();
    if ie_re.is_match(&ua_lower) {
        return Browser::Ie;
    }

    // Firefox detection (includes FxiOS for iOS)
    let firefox_re = Regex::new(r"firefox|fxios").unwrap();
    if firefox_re.is_match(&ua_lower) {
        return Browser::Firefox;
    }

    // Opera detection (uses Chromium, check before Chrome)
    let opera_re = Regex::new(r"opr/").unwrap();
    if opera_re.is_match(&ua_lower) {
        return Browser::Other;
    }

    // Chrome detection (includes CriOS for iOS)
    let chrome_re = Regex::new(r"chrome|crios").unwrap();
    if chrome_re.is_match(&ua_lower) {
        return Browser::Chrome;
    }

    // Safari detection (should be last as Chrome/Firefox on iOS also include Safari)
    let safari_re = Regex::new(r"safari").unwrap();
    if safari_re.is_match(&ua_lower) {
        return Browser::Safari;
    }

    Browser::Other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_edge_browser() {
        let edge_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
        let legacy_edge_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edge/89.0.774.57";
        assert_eq!(umt_extract_browser_from_user_agent(edge_ua), Browser::Edge);
        assert_eq!(
            umt_extract_browser_from_user_agent(legacy_edge_ua),
            Browser::Edge
        );
    }

    #[test]
    fn test_detect_chrome_browser() {
        let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
        assert_eq!(
            umt_extract_browser_from_user_agent(chrome_ua),
            Browser::Chrome
        );
    }

    #[test]
    fn test_detect_ios_chrome_browser() {
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
    fn test_detect_firefox_browser() {
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
    fn test_detect_safari_browser() {
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
    fn test_detect_internet_explorer() {
        let ie11_ua =
            "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
        let ie10_ua = "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; Trident/6.0)";
        assert_eq!(umt_extract_browser_from_user_agent(ie11_ua), Browser::Ie);
        assert_eq!(umt_extract_browser_from_user_agent(ie10_ua), Browser::Ie);
    }

    #[test]
    fn test_detect_other_browsers() {
        let opera_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 OPR/75.0.3969.149";
        let unknown_ua = "Unknown Browser";
        let empty_ua = "";
        assert_eq!(umt_extract_browser_from_user_agent(opera_ua), Browser::Other);
        assert_eq!(
            umt_extract_browser_from_user_agent(unknown_ua),
            Browser::Other
        );
        assert_eq!(umt_extract_browser_from_user_agent(empty_ua), Browser::Other);
    }
}
