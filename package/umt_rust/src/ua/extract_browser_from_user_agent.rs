use regex::Regex;
use std::sync::LazyLock;

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

static EDGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)edg(e)?").unwrap());
static IE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)msie|trident").unwrap());
static FIREFOX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)firefox|fxios").unwrap());
static OPERA_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)opr/").unwrap());
static CHROME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)chrome|crios").unwrap());
static SAFARI_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)safari").unwrap());

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
    // Edge detection (includes both "Edg" and "Edge")
    if EDGE_RE.is_match(ua) {
        return Browser::Edge;
    }

    // Internet Explorer detection
    if IE_RE.is_match(ua) {
        return Browser::Ie;
    }

    // Firefox detection (includes FxiOS for iOS)
    if FIREFOX_RE.is_match(ua) {
        return Browser::Firefox;
    }

    // Opera detection (uses Chromium, check before Chrome)
    if OPERA_RE.is_match(ua) {
        return Browser::Other;
    }

    // Chrome detection (includes CriOS for iOS)
    if CHROME_RE.is_match(ua) {
        return Browser::Chrome;
    }

    // Safari detection (should be last as Chrome/Firefox on iOS also include Safari)
    if SAFARI_RE.is_match(ua) {
        return Browser::Safari;
    }

    Browser::Other
}
