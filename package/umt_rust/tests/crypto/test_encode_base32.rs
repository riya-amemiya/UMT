use umt_rust::crypto::umt_encode_base32;

#[test]
fn test_encodes_a_simple_string() {
    assert_eq!(umt_encode_base32("Hello"), "JBSWY3DP");
}

#[test]
fn test_encodes_an_empty_string() {
    assert_eq!(umt_encode_base32(""), "");
}

#[test]
fn test_encodes_a_string_with_padding() {
    assert_eq!(umt_encode_base32("f"), "MY======");
    assert_eq!(umt_encode_base32("fo"), "MZXQ====");
    assert_eq!(umt_encode_base32("foo"), "MZXW6===");
    assert_eq!(umt_encode_base32("foob"), "MZXW6YQ=");
    assert_eq!(umt_encode_base32("fooba"), "MZXW6YTB");
    assert_eq!(umt_encode_base32("foobar"), "MZXW6YTBOI======");
}

#[test]
fn test_encodes_a_uint8array() {
    let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    assert_eq!(umt_encode_base32(bytes), "JBSWY3DP");
}

#[test]
fn test_encodes_special_characters() {
    assert_eq!(umt_encode_base32("こんにちは"), "4OAZHY4CSPRYDK7DQGQ6HANP");
}

#[test]
fn test_encodes_longer_text() {
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(
        umt_encode_base32(text),
        "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
    );
}

use umt_rust::crypto::*;

#[test]
fn test_encode_byte_slice() {
    let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    assert_eq!(umt_encode_base32(bytes), "JBSWY3DP");
}

#[test]
fn test_encode_empty_string() {
    assert_eq!(umt_encode_base32(""), "");
}

#[test]
fn test_encode_longer_text() {
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(
        umt_encode_base32(text),
        "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
    );
}

#[test]
fn test_encode_simple_string() {
    assert_eq!(umt_encode_base32("Hello"), "JBSWY3DP");
}

#[test]
fn test_encode_special_characters() {
    // UTF-8 bytes for Japanese characters
    assert_eq!(umt_encode_base32("こんにちは"), "4OAZHY4CSPRYDK7DQGQ6HANP");
}

#[test]
fn test_encode_with_padding() {
    assert_eq!(umt_encode_base32("f"), "MY======");
    assert_eq!(umt_encode_base32("fo"), "MZXQ====");
    assert_eq!(umt_encode_base32("foo"), "MZXW6===");
    assert_eq!(umt_encode_base32("foob"), "MZXW6YQ=");
    assert_eq!(umt_encode_base32("fooba"), "MZXW6YTB");
    assert_eq!(umt_encode_base32("foobar"), "MZXW6YTBOI======");
}
