use regex::Regex;

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
    let bot_re = Regex::new(r"bot|googlebot|crawler|spider|robot|crawling").unwrap();
    if bot_re.is_match(&ua_lower) {
        return Device::Bot;
    }

    // Mobile device detection (non-Android)
    let mobile_re = Regex::new(r"iphone|ipod|webos|blackberry|iemobile|opera mini").unwrap();
    if mobile_re.is_match(&ua_lower) {
        return Device::Mobile;
    }

    // Android device detection with mobile/tablet distinction
    let android_re = Regex::new(r"android").unwrap();
    if android_re.is_match(&ua_lower) {
        let mobile_check_re = Regex::new(r"mobile").unwrap();
        if mobile_check_re.is_match(&ua_lower) {
            return Device::Mobile;
        }
        return Device::Tablet;
    }

    // iPad detection (tablet)
    let ipad_re = Regex::new(r"ipad").unwrap();
    if ipad_re.is_match(&ua_lower) {
        return Device::Tablet;
    }

    // Desktop detection
    let desktop_re = Regex::new(r"windows|macintosh|linux").unwrap();
    if desktop_re.is_match(&ua_lower) {
        return Device::Desktop;
    }

    Device::Other
}
