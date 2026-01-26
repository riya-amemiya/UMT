use umt_rust::crypto::umt_encode_base58;

#[test]
fn test_encodes_a_simple_string() {
    assert_eq!(umt_encode_base58("Hello"), "9Ajdvzr");
}

#[test]
fn test_encodes_an_empty_string() {
    assert_eq!(umt_encode_base58(""), "");
}

#[test]
fn test_encodes_single_characters() {
    assert_eq!(umt_encode_base58("a"), "2g");
    assert_eq!(umt_encode_base58("b"), "2h");
    assert_eq!(umt_encode_base58("c"), "2i");
}

#[test]
fn test_encodes_a_uint8array() {
    let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    assert_eq!(umt_encode_base58(bytes), "9Ajdvzr");
}

#[test]
fn test_handles_leading_zeros() {
    let bytes: &[u8] = &[0, 0, 72, 101, 108, 108, 111];
    assert_eq!(umt_encode_base58(bytes), "119Ajdvzr");
}

#[test]
fn test_encodes_longer_text() {
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(
        umt_encode_base58(text),
        "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
    );
}

#[test]
fn test_encodes_special_characters() {
    assert_eq!(umt_encode_base58("こんにちは"), "7NAasPYBzpyEe5hmwr1KL");
}

#[test]
fn test_encodes_binary_data() {
    let bytes: &[u8] = &[255, 254, 253, 252, 251];
    assert_eq!(umt_encode_base58(bytes), "Vt9aq46");
}

use umt_rust::crypto::*;

#[test]
fn test_encode_binary_data() {
    let bytes: &[u8] = &[255, 254, 253, 252, 251];
    assert_eq!(umt_encode_base58(bytes), "Vt9aq46");
}

#[test]
fn test_encode_byte_slice() {
    let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    assert_eq!(umt_encode_base58(bytes), "9Ajdvzr");
}

#[test]
fn test_encode_empty_string() {
    assert_eq!(umt_encode_base58(""), "");
}

#[test]
fn test_encode_longer_text() {
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(
        umt_encode_base58(text),
        "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
    );
}

#[test]
fn test_encode_simple_string() {
    assert_eq!(umt_encode_base58("Hello"), "9Ajdvzr");
}

#[test]
fn test_encode_single_characters() {
    assert_eq!(umt_encode_base58("a"), "2g");
    assert_eq!(umt_encode_base58("b"), "2h");
    assert_eq!(umt_encode_base58("c"), "2i");
}

#[test]
fn test_encode_special_characters() {
    assert_eq!(umt_encode_base58("こんにちは"), "7NAasPYBzpyEe5hmwr1KL");
}

#[test]
fn test_encode_with_leading_zeros() {
    let bytes: &[u8] = &[0, 0, 72, 101, 108, 108, 111];
    assert_eq!(umt_encode_base58(bytes), "119Ajdvzr");
}
