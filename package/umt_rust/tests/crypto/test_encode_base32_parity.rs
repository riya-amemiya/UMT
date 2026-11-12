use umt_rust::crypto::umt_encode_base32;

#[test]
fn test_encode_base32_parity_empty_input() {
    assert_eq!(umt_encode_base32(""), "");
}

#[test]
fn test_encode_base32_parity_every_padding_length() {
    assert_eq!(umt_encode_base32("f"), "MY======");
    assert_eq!(umt_encode_base32("fo"), "MZXQ====");
    assert_eq!(umt_encode_base32("foo"), "MZXW6===");
    assert_eq!(umt_encode_base32("foob"), "MZXW6YQ=");
    assert_eq!(umt_encode_base32("fooba"), "MZXW6YTB");
    assert_eq!(umt_encode_base32("foobar"), "MZXW6YTBOI======");
}

#[test]
fn test_encode_base32_parity_binary_input() {
    assert_eq!(
        umt_encode_base32([255u8, 0, 128, 7, 99, 1]),
        "74AIAB3DAE======"
    );
}

#[test]
fn test_encode_base32_parity_output_length_is_a_multiple_of_eight() {
    for length in 0..=40usize {
        let encoded = umt_encode_base32(vec![0xA5u8; length]);
        assert_eq!(encoded.len(), length.div_ceil(5) * 8);
    }
}
