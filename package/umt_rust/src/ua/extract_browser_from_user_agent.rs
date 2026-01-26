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
