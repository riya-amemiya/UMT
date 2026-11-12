use umt_rust::string::umt_encode_base64_url;

#[test]
fn test_encodes_without_padding() {
    assert_eq!(umt_encode_base64_url("Hello World"), "SGVsbG8gV29ybGQ");
    assert_eq!(umt_encode_base64_url("f"), "Zg");
    assert_eq!(umt_encode_base64_url("fo"), "Zm8");
    assert_eq!(umt_encode_base64_url("foo"), "Zm9v");
}

#[test]
fn test_uses_url_safe_alphabet() {
    assert_eq!(umt_encode_base64_url("\u{00FF}\u{00FE}"), "w7_Dvg");
    assert_eq!(umt_encode_base64_url("\u{1F600}?>"), "8J-YgD8-");
}

#[test]
fn test_encodes_an_empty_string() {
    assert_eq!(umt_encode_base64_url(""), "");
}
