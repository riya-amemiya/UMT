use umt_rust::crypto::umt_decode_base32;

#[test]
fn test_decodes_a_simple_string() {
    let result = umt_decode_base32("JBSWY3DP").unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Hello");
}

#[test]
fn test_decodes_an_empty_string() {
    let result = umt_decode_base32("").unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_decodes_strings_with_padding() {
    assert_eq!(
        String::from_utf8(umt_decode_base32("MY======").unwrap()).unwrap(),
        "f"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXQ====").unwrap()).unwrap(),
        "fo"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXW6===").unwrap()).unwrap(),
        "foo"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXW6YQ=").unwrap()).unwrap(),
        "foob"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXW6YTB").unwrap()).unwrap(),
        "fooba"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXW6YTBOI======").unwrap()).unwrap(),
        "foobar"
    );
}

#[test]
fn test_throws_error_on_invalid_character_at_sign() {
    let result = umt_decode_base32("JBSWY3D@");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message,
        "Invalid base32 character: @"
    );
}

#[test]
fn test_throws_error_on_invalid_character_one() {
    let result = umt_decode_base32("JBSWY3D1");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message,
        "Invalid base32 character: 1"
    );
}

#[test]
fn test_throws_error_on_invalid_character_zero() {
    let result = umt_decode_base32("JBSWY3D0");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message,
        "Invalid base32 character: 0"
    );
}

#[test]
fn test_decodes_longer_text() {
    let encoded = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
    let result = String::from_utf8(umt_decode_base32(encoded).unwrap()).unwrap();
    assert_eq!(result, "The quick brown fox jumps over the lazy dog");
}

#[test]
fn test_handles_base32_without_padding() {
    assert_eq!(
        String::from_utf8(umt_decode_base32("JBSWY3DP").unwrap()).unwrap(),
        "Hello"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base32("MZXW6YTB").unwrap()).unwrap(),
        "fooba"
    );
}
