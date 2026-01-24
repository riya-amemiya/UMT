use super::extract_browser_from_user_agent::{umt_extract_browser_from_user_agent, Browser};
use super::extract_device_from_user_agent::{umt_extract_device_from_user_agent, Device};
use super::extract_os_from_user_agent::{umt_extract_os_from_user_agent, Os};

/// Represents simplified user agent information containing browser, device, and OS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAgentInfo {
    pub os: Os,
    pub browser: Browser,
    pub device: Device,
}

impl UserAgentInfo {
    /// Creates a new UserAgentInfo with the given components.
    pub fn new(os: Os, browser: Browser, device: Device) -> Self {
        Self { os, browser, device }
    }
}

/// Parses a User-Agent string to extract browser, device, and OS information.
///
/// # Arguments
///
/// * `user_agent` - The complete User-Agent string to analyze.
///
/// # Returns
///
/// A `UserAgentInfo` struct containing the detected browser, device, and operating system.
///
/// # Examples
///
/// ```
/// use umt_rust::ua::{umt_parse_user_agent, Browser, Device, Os};
///
/// let info = umt_parse_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/89.0 Safari/537.36");
/// assert_eq!(info.os, Os::Windows);
/// assert_eq!(info.browser, Browser::Chrome);
/// assert_eq!(info.device, Device::Desktop);
/// ```
#[inline]
pub fn umt_parse_user_agent(user_agent: &str) -> UserAgentInfo {
    let ua = user_agent.to_lowercase();

    UserAgentInfo {
        os: umt_extract_os_from_user_agent(&ua),
        browser: umt_extract_browser_from_user_agent(&ua),
        device: umt_extract_device_from_user_agent(&ua),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_mobile_with_safari() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Ios);
        assert_eq!(result.browser, Browser::Safari);
        assert_eq!(result.device, Device::Mobile);
    }

    #[test]
    fn test_android_mobile_with_chrome() {
        let ua = "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Android);
        assert_eq!(result.browser, Browser::Chrome);
        assert_eq!(result.device, Device::Mobile);
    }

    #[test]
    fn test_macos_desktop_with_firefox() {
        let ua =
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:88.0) Gecko/20100101 Firefox/88.0";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::MacOs);
        assert_eq!(result.browser, Browser::Firefox);
        assert_eq!(result.device, Device::Desktop);
    }

    #[test]
    fn test_windows_desktop_with_edge() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Windows);
        assert_eq!(result.browser, Browser::Edge);
        assert_eq!(result.device, Device::Desktop);
    }

    #[test]
    fn test_linux_desktop_with_chrome() {
        let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Linux);
        assert_eq!(result.browser, Browser::Chrome);
        assert_eq!(result.device, Device::Desktop);
    }

    #[test]
    fn test_ipad_as_tablet() {
        let ua = "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Ios);
        assert_eq!(result.browser, Browser::Safari);
        assert_eq!(result.device, Device::Tablet);
    }

    #[test]
    fn test_android_tablet() {
        let ua = "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Android);
        assert_eq!(result.browser, Browser::Chrome);
        assert_eq!(result.device, Device::Tablet);
    }

    #[test]
    fn test_internet_explorer_on_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Windows);
        assert_eq!(result.browser, Browser::Ie);
        assert_eq!(result.device, Device::Desktop);
    }

    #[test]
    fn test_googlebot() {
        let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Other);
        assert_eq!(result.browser, Browser::Other);
        assert_eq!(result.device, Device::Bot);
    }

    #[test]
    fn test_unknown_user_agent() {
        let ua = "Unknown User Agent String";
        let result = umt_parse_user_agent(ua);
        assert_eq!(result.os, Os::Other);
        assert_eq!(result.browser, Browser::Other);
        assert_eq!(result.device, Device::Other);
    }
}
