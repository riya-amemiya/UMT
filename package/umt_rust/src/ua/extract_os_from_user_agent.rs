use regex::Regex;

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
    let ua_lower = ua.to_lowercase();

    // iOS detection (iPhone, iPad, iPod)
    let ios_re = Regex::new(r"iphone|ipad|ipod").unwrap();
    if ios_re.is_match(&ua_lower) {
        return Os::Ios;
    }

    // Android detection
    let android_re = Regex::new(r"android").unwrap();
    if android_re.is_match(&ua_lower) {
        return Os::Android;
    }

    // macOS detection
    let macos_re = Regex::new(r"mac os x").unwrap();
    if macos_re.is_match(&ua_lower) {
        return Os::MacOs;
    }

    // Windows detection
    let windows_re = Regex::new(r"windows|win32").unwrap();
    if windows_re.is_match(&ua_lower) {
        return Os::Windows;
    }

    // Linux detection
    let linux_re = Regex::new(r"linux").unwrap();
    if linux_re.is_match(&ua_lower) {
        return Os::Linux;
    }

    Os::Other
}
