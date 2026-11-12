use regex::Regex;
use std::sync::LazyLock;

/// Represents the detected operating system from a User-Agent string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Os {
    Ios,
    Android,
    MacOs,
    Windows,
    Linux,
    Other,
}

impl Os {
    /// Converts the Os enum to its string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Os::Ios => "ios",
            Os::Android => "android",
            Os::MacOs => "macos",
            Os::Windows => "windows",
            Os::Linux => "linux",
            Os::Other => "other",
        }
    }
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

static IOS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)iphone|ipad|ipod").unwrap());
static ANDROID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)android").unwrap());
static MACOS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)mac os x").unwrap());
static WINDOWS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)windows|win32").unwrap());
static LINUX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i-u)linux").unwrap());

/// Extracts operating system information from a User-Agent string.
///
/// # Arguments
///
/// * `ua` - The User-Agent string to analyze.
///
/// # Returns
///
/// The detected operating system (iOS, Android, macOS, Windows, Linux, or Other).
///
/// # Examples
///
/// ```
/// use umt_rust::ua::umt_extract_os_from_user_agent;
///
/// let os = umt_extract_os_from_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)");
/// assert_eq!(os.as_str(), "windows");
/// ```
#[inline]
pub fn umt_extract_os_from_user_agent(ua: &str) -> Os {
    // iOS detection (iPhone, iPad, iPod)
    if IOS_RE.is_match(ua) {
        return Os::Ios;
    }

    // Android detection
    if ANDROID_RE.is_match(ua) {
        return Os::Android;
    }

    // macOS detection
    if MACOS_RE.is_match(ua) {
        return Os::MacOs;
    }

    // Windows detection
    if WINDOWS_RE.is_match(ua) {
        return Os::Windows;
    }

    // Linux detection
    if LINUX_RE.is_match(ua) {
        return Os::Linux;
    }

    Os::Other
}
