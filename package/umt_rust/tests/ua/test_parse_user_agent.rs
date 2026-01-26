use umt_rust::ua::{Browser, Device, Os, umt_parse_user_agent};

#[test]
fn test_should_detect_ios_mobile_with_safari() {
    let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Ios);
    assert_eq!(result.browser, Browser::Safari);
    assert_eq!(result.device, Device::Mobile);
}

#[test]
fn test_should_detect_android_mobile_with_chrome() {
    let ua = "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Android);
    assert_eq!(result.browser, Browser::Chrome);
    assert_eq!(result.device, Device::Mobile);
}

#[test]
fn test_should_detect_macos_desktop_with_firefox() {
    let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:88.0) Gecko/20100101 Firefox/88.0";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::MacOs);
    assert_eq!(result.browser, Browser::Firefox);
    assert_eq!(result.device, Device::Desktop);
}

#[test]
fn test_should_detect_windows_desktop_with_edge() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Windows);
    assert_eq!(result.browser, Browser::Edge);
    assert_eq!(result.device, Device::Desktop);
}

#[test]
fn test_should_detect_linux_desktop_with_chrome() {
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Linux);
    assert_eq!(result.browser, Browser::Chrome);
    assert_eq!(result.device, Device::Desktop);
}

#[test]
fn test_should_detect_ipad_as_tablet() {
    let ua = "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Ios);
    assert_eq!(result.browser, Browser::Safari);
    assert_eq!(result.device, Device::Tablet);
}

#[test]
fn test_should_detect_android_tablet() {
    let ua = "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Android);
    assert_eq!(result.browser, Browser::Chrome);
    assert_eq!(result.device, Device::Tablet);
}

#[test]
fn test_should_detect_internet_explorer_on_windows() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Windows);
    assert_eq!(result.browser, Browser::Ie);
    assert_eq!(result.device, Device::Desktop);
}

#[test]
fn test_should_detect_googlebot() {
    let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Other);
    assert_eq!(result.browser, Browser::Other);
    assert_eq!(result.device, Device::Bot);
}

#[test]
fn test_should_handle_unknown_user_agent() {
    let ua = "Unknown User Agent String";
    let result = umt_parse_user_agent(ua);
    assert_eq!(result.os, Os::Other);
    assert_eq!(result.browser, Browser::Other);
    assert_eq!(result.device, Device::Other);
}

#[test]
fn test_user_agent_info_has_correct_fields() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/89.0";
    let result = umt_parse_user_agent(ua);

    // Access all public fields
    let _ = result.os;
    let _ = result.browser;
    let _ = result.device;
}

#[test]
fn test_user_agent_info_new() {
    use umt_rust::ua::UserAgentInfo;

    let info = UserAgentInfo::new(Os::Windows, Browser::Chrome, Device::Desktop);
    assert_eq!(info.os, Os::Windows);
    assert_eq!(info.browser, Browser::Chrome);
    assert_eq!(info.device, Device::Desktop);
}

#[test]
fn test_user_agent_info_clone() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/89.0";
    let result = umt_parse_user_agent(ua);
    let cloned = result.clone();
    assert_eq!(result, cloned);
}

#[test]
fn test_user_agent_info_debug() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/89.0";
    let result = umt_parse_user_agent(ua);
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("UserAgentInfo"));
}
