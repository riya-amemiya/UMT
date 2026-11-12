use umt_rust::ua::{Os, umt_extract_os_from_user_agent};

#[test]
fn test_os_parity_case_folding_is_ascii_only() {
    assert_eq!(umt_extract_os_from_user_agent("w\u{0131}ndows"), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent("\u{0130}phone"), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent("mac o\u{017F} x"), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent("l\u{0131}nux"), Os::Other);
    assert_eq!(umt_extract_os_from_user_agent("andro\u{0131}d"), Os::Other);
}

#[test]
fn test_os_parity_ascii_case_is_ignored_for_every_os() {
    assert_eq!(umt_extract_os_from_user_agent("IPOD"), Os::Ios);
    assert_eq!(umt_extract_os_from_user_agent("ANDROID"), Os::Android);
    assert_eq!(umt_extract_os_from_user_agent("MAC OS X"), Os::MacOs);
    assert_eq!(umt_extract_os_from_user_agent("WIN32"), Os::Windows);
    assert_eq!(umt_extract_os_from_user_agent("LINUX"), Os::Linux);
    assert_eq!(umt_extract_os_from_user_agent(""), Os::Other);
}
