use umt_rust::ua::{Device, umt_extract_device_from_user_agent};

#[test]
fn test_device_parity_case_folding_is_ascii_only() {
    assert_eq!(
        umt_extract_device_from_user_agent("blac\u{212A}berry"),
        Device::Other
    );
    assert_eq!(
        umt_extract_device_from_user_agent("w\u{0131}ndows"),
        Device::Other
    );
    assert_eq!(
        umt_extract_device_from_user_agent("\u{0130}phone"),
        Device::Other
    );
    assert_eq!(
        umt_extract_device_from_user_agent("\u{017F}pider"),
        Device::Other
    );
    assert_eq!(
        umt_extract_device_from_user_agent("andro\u{0131}d \u{0130}pad"),
        Device::Other
    );
}

#[test]
fn test_device_parity_ascii_case_is_ignored_for_every_device() {
    assert_eq!(umt_extract_device_from_user_agent("GOOGLEBOT"), Device::Bot);
    assert_eq!(
        umt_extract_device_from_user_agent("OPERA MINI"),
        Device::Mobile
    );
    assert_eq!(
        umt_extract_device_from_user_agent("ANDROID MOBILE"),
        Device::Mobile
    );
    assert_eq!(
        umt_extract_device_from_user_agent("ANDROID"),
        Device::Tablet
    );
    assert_eq!(umt_extract_device_from_user_agent("IPAD"), Device::Tablet);
    assert_eq!(
        umt_extract_device_from_user_agent("MACINTOSH"),
        Device::Desktop
    );
    assert_eq!(umt_extract_device_from_user_agent(""), Device::Other);
}
