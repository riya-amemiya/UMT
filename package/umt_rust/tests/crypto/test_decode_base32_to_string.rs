use umt_rust::crypto::umt_decode_base32_to_string;

#[test]
fn test_decodes_a_simple_string() {
    assert_eq!(umt_decode_base32_to_string("JBSWY3DP").unwrap(), "Hello");
}

#[test]
fn test_decodes_an_empty_string() {
    assert_eq!(umt_decode_base32_to_string("").unwrap(), "");
}

#[test]
fn test_decodes_strings_with_padding() {
    assert_eq!(umt_decode_base32_to_string("MY======").unwrap(), "f");
    assert_eq!(umt_decode_base32_to_string("MZXQ====").unwrap(), "fo");
    assert_eq!(umt_decode_base32_to_string("MZXW6===").unwrap(), "foo");
    assert_eq!(umt_decode_base32_to_string("MZXW6YQ=").unwrap(), "foob");
    assert_eq!(umt_decode_base32_to_string("MZXW6YTB").unwrap(), "fooba");
    assert_eq!(umt_decode_base32_to_string("MZXW6YTBOI======").unwrap(), "foobar");
}

#[test]
fn test_decodes_special_characters() {
    assert_eq!(
        umt_decode_base32_to_string("4OAZHY4CSPRYDK7DQGQ6HANP").unwrap(),
        "こんにちは"
    );
}

#[test]
fn test_throws_error_on_invalid_character() {
    let result = umt_decode_base32_to_string("JBSWY3D@");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message,
        "Invalid base32 character: @"
    );
}

#[test]
fn test_decodes_longer_text() {
    let encoded = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
    assert_eq!(
        umt_decode_base32_to_string(encoded).unwrap(),
        "The quick brown fox jumps over the lazy dog"
    );
}
