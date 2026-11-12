use umt_rust::string::{umt_decode_base64_url, umt_encode_base64_url};

#[test]
fn test_decodes_input_without_padding() {
    assert_eq!(
        umt_decode_base64_url("SGVsbG8gV29ybGQ").unwrap(),
        "Hello World"
    );
    assert_eq!(umt_decode_base64_url("Zg").unwrap(), "f");
    assert_eq!(umt_decode_base64_url("Zm8").unwrap(), "fo");
    assert_eq!(umt_decode_base64_url("Zm9v").unwrap(), "foo");
}

#[test]
fn test_decodes_input_with_padding() {
    assert_eq!(
        umt_decode_base64_url("SGVsbG8gV29ybGQ=").unwrap(),
        "Hello World"
    );
    assert_eq!(umt_decode_base64_url("Zg==").unwrap(), "f");
    assert_eq!(umt_decode_base64_url("Zg=").unwrap(), "f");
}

#[test]
fn test_decodes_url_safe_alphabet() {
    assert_eq!(umt_decode_base64_url("w7_Dvg").unwrap(), "\u{00FF}\u{00FE}");
    assert_eq!(umt_decode_base64_url("8J-YgD8-").unwrap(), "\u{1F600}?>");
}

#[test]
fn test_ignores_trailing_bits() {
    assert_eq!(umt_decode_base64_url("Zh").unwrap(), "f");
}

#[test]
fn test_decodes_an_empty_string() {
    assert_eq!(umt_decode_base64_url("").unwrap(), "");
}

#[test]
fn test_round_trips_with_encode_base64_url() {
    let value = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F} ~?>/+";
    assert_eq!(
        umt_decode_base64_url(&umt_encode_base64_url(value)).unwrap(),
        value
    );
}

#[test]
fn test_rejects_invalid_input() {
    for value in [
        "Z", "Zm9vY", "Zg===", "Zm9v=", "Z===", "w7/Dvg", "w7+Dvg", "Zg ==", "Zg\n", "=", "Z=g",
    ] {
        assert_eq!(
            umt_decode_base64_url(value),
            Err("Invalid Base64 string".to_string()),
            "{:?}",
            value
        );
    }
}

#[test]
fn test_rejects_invalid_utf8() {
    assert_eq!(
        umt_decode_base64_url("_w"),
        Err("Invalid UTF-8 sequence".to_string())
    );
}
