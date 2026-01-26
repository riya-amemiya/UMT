use umt_rust::ua::{Device, umt_extract_device_from_user_agent};

#[test]
fn test_should_detect_bots_and_crawlers() {
    let googlebot = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    let bingbot = "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)";
    let crawler = "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)";

    assert_eq!(umt_extract_device_from_user_agent(googlebot), Device::Bot);
    assert_eq!(umt_extract_device_from_user_agent(bingbot), Device::Bot);
    assert_eq!(umt_extract_device_from_user_agent(crawler), Device::Bot);
}

#[test]
fn test_should_detect_mobile_devices() {
    let iphone = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    let android_mobile = "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    let blackberry = "Mozilla/5.0 (BlackBerry; U; BlackBerry 9900; en) AppleWebKit/534.11+ (KHTML, like Gecko) Version/7.1.0.346 Mobile Safari/534.11+";
    let opera_mini =
        "Opera/9.80 (J2ME/MIDP; Opera Mini/5.1.21214/28.2725; U; ru) Presto/2.8.119 Version/11.10";

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
fn test_should_detect_tablets() {
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
fn test_should_detect_desktop_devices() {
    let windows = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    let mac = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    let linux = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    assert_eq!(umt_extract_device_from_user_agent(windows), Device::Desktop);
    assert_eq!(umt_extract_device_from_user_agent(mac), Device::Desktop);
    assert_eq!(umt_extract_device_from_user_agent(linux), Device::Desktop);
}

#[test]
fn test_should_handle_unknown_devices() {
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
fn test_should_handle_android_edge_cases() {
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

#[test]
fn test_device_as_str_method() {
    assert_eq!(Device::Bot.as_str(), "bot");
    assert_eq!(Device::Mobile.as_str(), "mobile");
    assert_eq!(Device::Tablet.as_str(), "tablet");
    assert_eq!(Device::Desktop.as_str(), "desktop");
    assert_eq!(Device::Other.as_str(), "other");
}
