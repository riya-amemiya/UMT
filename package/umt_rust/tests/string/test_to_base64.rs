use umt_rust::string::umt_to_base64;

#[test]
fn test_convert_normal_string_to_base64() {
    assert_eq!(umt_to_base64("test"), "dGVzdA==");
}

#[test]
fn test_convert_empty_string_to_base64() {
    assert_eq!(umt_to_base64(""), "");
}

#[test]
fn test_convert_string_with_special_characters_to_base64() {
    assert_eq!(umt_to_base64("@#%"), "QCMl");
}

#[test]
fn test_convert_string_with_japanese_characters_to_base64() {
    assert_eq!(umt_to_base64("あいうえお"), "44GC44GE44GG44GI44GK");
}

#[test]
fn test_convert_long_string_to_base64() {
    assert_eq!(
        umt_to_base64("This is a long string to test the Base64 conversion"),
        "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u"
    );
}
