use umt_rust::crypto::{HexError, umt_decode_hex, umt_encode_hex};

#[test]
fn test_decodes_lowercase_hex() {
    assert_eq!(umt_decode_hex("48656c6c6f").unwrap(), b"Hello");
}

#[test]
fn test_decodes_uppercase_hex() {
    assert_eq!(
        umt_decode_hex("00010F10ABFF").unwrap(),
        vec![0, 1, 15, 16, 171, 255]
    );
}

#[test]
fn test_decodes_an_empty_string() {
    assert_eq!(umt_decode_hex("").unwrap(), Vec::<u8>::new());
}

#[test]
fn test_round_trips_with_encode_hex() {
    let bytes: Vec<u8> = (0..=255).collect();
    assert_eq!(umt_decode_hex(&umt_encode_hex(&bytes)).unwrap(), bytes);
}

#[test]
fn test_rejects_invalid_input() {
    for value in [
        "0",
        "abc",
        "zz",
        "0g",
        "g0",
        "00 ff",
        " 00",
        "00\n",
        "\u{FF10}\u{FF10}",
    ] {
        let error = umt_decode_hex(value).unwrap_err();
        assert_eq!(
            error,
            HexError {
                message: "Invalid hex string".to_string()
            },
            "{:?}",
            value
        );
        assert_eq!(error.to_string(), "Invalid hex string");
    }
}
