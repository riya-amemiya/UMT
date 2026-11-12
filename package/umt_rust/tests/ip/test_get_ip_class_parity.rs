use umt_rust::ip::get_ip_class;

#[test]
fn test_get_ip_class_parity_first_octet_stops_at_first_non_digit() {
    assert_eq!(get_ip_class("10abc.0.0.1"), "A");
    assert_eq!(get_ip_class("1e2.0.0.1"), "A");
    assert_eq!(get_ip_class("12 3.0.0.1"), "A");
    assert_eq!(get_ip_class("1_0.0.0.1"), "A");
    assert_eq!(get_ip_class("300abc.0.0.1"), "");
}

#[test]
fn test_get_ip_class_parity_leading_javascript_whitespace_is_skipped() {
    assert_eq!(get_ip_class(" 10.0.0.1"), "A");
    assert_eq!(get_ip_class("\t127.0.0.1"), "A");
    assert_eq!(get_ip_class("\n\u{000B}\u{000C}\r128.0.0.1"), "B");
    assert_eq!(get_ip_class("\u{00A0}200.1.1.1"), "C");
    assert_eq!(get_ip_class("\u{FEFF}200.1.1.1"), "C");
    assert_eq!(get_ip_class("\u{1680}\u{2000}\u{200A}200.1.1.1"), "C");
    assert_eq!(get_ip_class("\u{3000}224.1.1.1"), "D");
    assert_eq!(get_ip_class("\u{202F}\u{205F}224.1.1.1"), "D");
    assert_eq!(get_ip_class("\u{2028}\u{2029}240.1.1.1"), "E");
}

#[test]
fn test_get_ip_class_parity_non_javascript_whitespace_is_not_skipped() {
    assert_eq!(get_ip_class("\u{0085}240.1.1.1"), "");
    assert_eq!(get_ip_class("\u{180E}240.1.1.1"), "");
}

#[test]
fn test_get_ip_class_parity_sign_handling() {
    assert_eq!(get_ip_class("+10.0.0.1"), "A");
    assert_eq!(get_ip_class("-5.0.0.1"), "");
    assert_eq!(get_ip_class("-0.0.0.0"), "");
    assert_eq!(get_ip_class("+ 5.0.0.1"), "");
    assert_eq!(get_ip_class("+.0.0.1"), "");
}

#[test]
fn test_get_ip_class_parity_non_ascii_digits_are_not_numbers() {
    assert_eq!(get_ip_class("\u{FF11}\u{FF10}.0.0.1"), "");
    assert_eq!(get_ip_class("\u{0663}.0.0.1"), "");
}

#[test]
fn test_get_ip_class_parity_missing_or_oversized_first_octet() {
    assert_eq!(get_ip_class(".0.0.1"), "");
    assert_eq!(get_ip_class("0x10.0.0.1"), "");
    assert_eq!(get_ip_class("99999999999999999999.0.0.1"), "");
    assert_eq!(get_ip_class("256.0.0.1"), "");
}

#[test]
fn test_get_ip_class_parity_class_boundaries() {
    assert_eq!(get_ip_class("010.0.0.1"), "A");
    assert_eq!(get_ip_class("127.0.0.1"), "A");
    assert_eq!(get_ip_class("128.0.0.1"), "B");
    assert_eq!(get_ip_class("191.0.0.1"), "B");
    assert_eq!(get_ip_class("192.0.0.1"), "C");
    assert_eq!(get_ip_class("223.0.0.1"), "C");
    assert_eq!(get_ip_class("224.0.0.1"), "D");
    assert_eq!(get_ip_class("239.0.0.1"), "D");
    assert_eq!(get_ip_class("240.0.0.1"), "E");
    assert_eq!(get_ip_class("255.0.0.1"), "E");
}

#[test]
fn test_get_ip_class_parity_empty_and_wrong_segment_count() {
    assert_eq!(get_ip_class(""), "");
    assert_eq!(get_ip_class("10.0.0"), "");
    assert_eq!(get_ip_class("10.0.0.1.1"), "");
}
