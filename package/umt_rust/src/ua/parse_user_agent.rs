use super::extract_browser_from_user_agent::{Browser, umt_extract_browser_from_user_agent};
use super::extract_device_from_user_agent::{Device, umt_extract_device_from_user_agent};
use super::extract_os_from_user_agent::{Os, umt_extract_os_from_user_agent};

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
        Self {
            os,
            browser,
            device,
        }
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
