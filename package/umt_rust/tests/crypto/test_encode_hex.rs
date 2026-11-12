use umt_rust::crypto::umt_encode_hex;

#[test]
fn test_encodes_a_string_as_utf8() {
    assert_eq!(umt_encode_hex("Hello"), "48656c6c6f");
}

#[test]
fn test_encodes_multi_byte_characters() {
    assert_eq!(umt_encode_hex("\u{3042}"), "e38182");
    assert_eq!(umt_encode_hex("\u{1F600}"), "f09f9880");
}

#[test]
fn test_encodes_bytes_with_lowercase_digits_and_zero_padding() {
    assert_eq!(umt_encode_hex([0u8, 1, 15, 16, 171, 255]), "00010f10abff");
}

#[test]
fn test_encodes_empty_input() {
    assert_eq!(umt_encode_hex(""), "");
    assert_eq!(umt_encode_hex(Vec::<u8>::new()), "");
}
