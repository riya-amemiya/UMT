use regex::Regex;
use std::sync::LazyLock;

/// Represents the detected device type from a User-Agent string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Device {
    Bot,
    Mobile,
    Tablet,
    Desktop,
    Other,
}

impl Device {
    /// Converts the Device enum to its string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Device::Bot => "bot",
            Device::Mobile => "mobile",
            Device::Tablet => "tablet",
            Device::Desktop => "desktop",
            Device::Other => "other",
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

static BOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"bot|googlebot|crawler|spider|robot|crawling").unwrap());
static MOBILE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"iphone|ipod|webos|blackberry|iemobile|opera mini").unwrap());
static ANDROID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"android").unwrap());
static MOBILE_CHECK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mobile").unwrap());
static IPAD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ipad").unwrap());
static DESKTOP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"windows|macintosh|linux").unwrap());

/// Extracts device type information from a User-Agent string.
///
/// # Arguments
///
/// * `ua` - The User-Agent string to analyze.
///
/// # Returns
///
/// The detected device type (Bot, Mobile, Tablet, Desktop, or Other).
///
/// # Examples
///
/// ```
/// use umt_rust::ua::umt_extract_device_from_user_agent;
///
/// let device = umt_extract_device_from_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)");
/// assert_eq!(device.as_str(), "desktop");
/// ```
#[inline]
pub fn umt_extract_device_from_user_agent(ua: &str) -> Device {
    let ua_lower = ua.to_lowercase();

    // Bot/crawler detection
    if BOT_RE.is_match(&ua_lower) {
        return Device::Bot;
    }

    // Mobile device detection (non-Android)
    if MOBILE_RE.is_match(&ua_lower) {
        return Device::Mobile;
    }

    // Android device detection with mobile/tablet distinction
    if ANDROID_RE.is_match(&ua_lower) {
        if MOBILE_CHECK_RE.is_match(&ua_lower) {
            return Device::Mobile;
        }
        return Device::Tablet;
    }

    // iPad detection (tablet)
    if IPAD_RE.is_match(&ua_lower) {
        return Device::Tablet;
    }

    // Desktop detection
    if DESKTOP_RE.is_match(&ua_lower) {
        return Device::Desktop;
    }

    Device::Other
}
