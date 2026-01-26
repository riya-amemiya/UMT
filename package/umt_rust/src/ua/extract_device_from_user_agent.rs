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
    if ua_lower.contains("bot")
        || ua_lower.contains("googlebot")
        || ua_lower.contains("crawler")
        || ua_lower.contains("spider")
        || ua_lower.contains("robot")
        || ua_lower.contains("crawling")
    {
        return Device::Bot;
    }

    // Mobile device detection (non-Android)
    if ua_lower.contains("iphone")
        || ua_lower.contains("ipod")
        || ua_lower.contains("webos")
        || ua_lower.contains("blackberry")
        || ua_lower.contains("iemobile")
        || ua_lower.contains("opera mini")
    {
        return Device::Mobile;
    }

    // Android device detection with mobile/tablet distinction
    if ua_lower.contains("android") {
        if ua_lower.contains("mobile") {
            return Device::Mobile;
        }
        return Device::Tablet;
    }

    // iPad detection (tablet)
    if ua_lower.contains("ipad") {
        return Device::Tablet;
    }

    // Desktop detection
    if ua_lower.contains("windows") || ua_lower.contains("macintosh") || ua_lower.contains("linux")
    {
        return Device::Desktop;
    }

    Device::Other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bots_and_crawlers() {
        let googlebot = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let bingbot = "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)";
        let crawler = "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)";

        assert_eq!(umt_extract_device_from_user_agent(googlebot), Device::Bot);
        assert_eq!(umt_extract_device_from_user_agent(bingbot), Device::Bot);
        assert_eq!(umt_extract_device_from_user_agent(crawler), Device::Bot);
    }

    #[test]
    fn test_detect_mobile_devices() {
        let iphone = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
        let android_mobile = "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
        let blackberry = "Mozilla/5.0 (BlackBerry; U; BlackBerry 9900; en) AppleWebKit/534.11+ (KHTML, like Gecko) Version/7.1.0.346 Mobile Safari/534.11+";
        let opera_mini = "Opera/9.80 (J2ME/MIDP; Opera Mini/5.1.21214/28.2725; U; ru) Presto/2.8.119 Version/11.10";

        assert_eq!(umt_extract_device_from_user_agent(iphone), Device::Mobile);
        assert_eq!(
            umt_extract_device_from_user_agent(android_mobile),
            Device::Mobile
        );
        assert_eq!(
            umt_extract_device_from_user_agent(blackberry),
            Device::Mobile
        );
        assert_eq!(
            umt_extract_device_from_user_agent(opera_mini),
            Device::Mobile
        );
    }

    #[test]
    fn test_detect_tablets() {
        let ipad = "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
        let android_tablet = "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
        let android_non_mobile = "Mozilla/5.0 (Linux; Android 11; SAMSUNG-SM-T377A Build/NMF26X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

        assert_eq!(umt_extract_device_from_user_agent(ipad), Device::Tablet);
        assert_eq!(
            umt_extract_device_from_user_agent(android_tablet),
            Device::Tablet
        );
        assert_eq!(
            umt_extract_device_from_user_agent(android_non_mobile),
            Device::Tablet
        );
    }

    #[test]
    fn test_detect_desktop_devices() {
        let windows = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
        let mac = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
        let linux = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

        assert_eq!(umt_extract_device_from_user_agent(windows), Device::Desktop);
        assert_eq!(umt_extract_device_from_user_agent(mac), Device::Desktop);
        assert_eq!(umt_extract_device_from_user_agent(linux), Device::Desktop);
    }

    #[test]
    fn test_detect_unknown_devices() {
        let empty = "";
        let unknown = "Unknown Device";
        let custom_device = "Mozilla/5.0 (CustomOS; Device) CustomBrowser/1.0";

        assert_eq!(umt_extract_device_from_user_agent(empty), Device::Other);
        assert_eq!(umt_extract_device_from_user_agent(unknown), Device::Other);
        assert_eq!(
            umt_extract_device_from_user_agent(custom_device),
            Device::Other
        );
    }

    #[test]
    fn test_android_edge_cases() {
        // Android device with explicit mobile flag
        let android_mobile = "Mozilla/5.0 (Linux; Android 10; Mobile) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
        // Android device without mobile flag (should be detected as tablet)
        let android_non_mobile = "Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

        assert_eq!(
            umt_extract_device_from_user_agent(android_mobile),
            Device::Mobile
        );
        assert_eq!(
            umt_extract_device_from_user_agent(android_non_mobile),
            Device::Tablet
        );
    }
}
