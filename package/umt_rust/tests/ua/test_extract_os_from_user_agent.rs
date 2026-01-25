use umt_rust::ua::{umt_extract_os_from_user_agent, Os};

#[test]
fn test_should_detect_ios_devices() {
    let iphone =
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1";
    let ipad =
        "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";
    let ipod =
        "Mozilla/5.0 (iPod touch; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1";

    assert_eq!(umt_extract_os_from_user_agent(iphone), Os::Ios);
    assert_eq!(umt_extract_os_from_user_agent(ipad), Os::Ios);
    assert_eq!(umt_extract_os_from_user_agent(ipod), Os::Ios);
}

#[test]
fn test_should_detect_android_devices() {
    let android_phone =
        "Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Mobile Safari/537.36";
    let android_tablet =
        "Mozilla/5.0 (Linux; Android 11; SM-T870) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";
    let android_tv =
        "Mozilla/5.0 (Linux; Android 10; AndroidTV) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.105 Safari/537.36";

    assert_eq!(umt_extract_os_from_user_agent(android_phone), Os::Android);
    assert_eq!(umt_extract_os_from_user_agent(android_tablet), Os::Android);
    assert_eq!(umt_extract_os_from_user_agent(android_tv), Os::Android);
}

#[test]
fn test_should_detect_macos_devices() {
    let macos_safari =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Safari/605.1.15";
    let macos_chrome =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    let macos_firefox =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:88.0) Gecko/20100101 Firefox/88.0";

    assert_eq!(umt_extract_os_from_user_agent(macos_safari), Os::MacOs);
    assert_eq!(umt_extract_os_from_user_agent(macos_chrome), Os::MacOs);
    assert_eq!(umt_extract_os_from_user_agent(macos_firefox), Os::MacOs);
}

#[test]
fn test_should_detect_windows_devices() {
    let win10_edge =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36 Edg/89.0.774.57";
    let win10_chrome =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    let win7_ie = "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko";
    let win32_ua =
        "Mozilla/5.0 (Win32; x86) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    assert_eq!(umt_extract_os_from_user_agent(win10_edge), Os::Windows);
    assert_eq!(umt_extract_os_from_user_agent(win10_chrome), Os::Windows);
    assert_eq!(umt_extract_os_from_user_agent(win7_ie), Os::Windows);
    assert_eq!(umt_extract_os_from_user_agent(win32_ua), Os::Windows);
}

#[test]
fn test_should_detect_linux_devices() {
    let ubuntu_chrome =
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";
    let fedora_firefox =
        "Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0";
    let generic_linux =
        "Mozilla/5.0 (Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

    assert_eq!(umt_extract_os_from_user_agent(ubuntu_chrome), Os::Linux);
    assert_eq!(umt_extract_os_from_user_agent(fedora_firefox), Os::Linux);
    assert_eq!(umt_extract_os_from_user_agent(generic_linux), Os::Linux);
}

#[test]
fn test_should_handle_unknown_operating_systems() {
    let empty = "";
    let unknown = "Unknown OS";
    let custom_os = "Mozilla/5.0 (CustomOS) CustomBrowser/1.0";
    let bot = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";

    assert_eq!(umt_extract_os_from_user_agent(empty), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent(unknown), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent(custom_os), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent(bot), Os::Other);
}

#[test]
fn test_os_as_str_method() {
    assert_eq!(Os::Ios.as_str(), "ios");
    assert_eq!(Os::Android.as_str(), "android");
    assert_eq!(Os::MacOs.as_str(), "macos");
    assert_eq!(Os::Windows.as_str(), "windows");
    assert_eq!(Os::Linux.as_str(), "linux");
    assert_eq!(Os::Other.as_str(), "other");
}
