use umt_rust::crypto::umt_decode_base58_to_string;

#[test]
fn test_decodes_a_simple_string() {
    assert_eq!(umt_decode_base58_to_string("9Ajdvzr").unwrap(), "Hello");
}

#[test]
fn test_decodes_an_empty_string() {
    assert_eq!(umt_decode_base58_to_string("").unwrap(), "");
}

#[test]
fn test_decodes_single_character_strings() {
    assert_eq!(umt_decode_base58_to_string("2g").unwrap(), "a");
    assert_eq!(umt_decode_base58_to_string("2h").unwrap(), "b");
    assert_eq!(umt_decode_base58_to_string("2i").unwrap(), "c");
}

#[test]
fn test_decodes_special_characters() {
    assert_eq!(
        umt_decode_base58_to_string("7NAasPYBzpyEe5hmwr1KL").unwrap(),
        "こんにちは"
    );
}

#[test]
fn test_throws_error_on_invalid_character_zero() {
    let result = umt_decode_base58_to_string("9Ajdvz0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: 0");
}

#[test]
fn test_throws_error_on_invalid_character_uppercase_o() {
    let result = umt_decode_base58_to_string("9AjdvzO");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: O");
}

#[test]
fn test_decodes_longer_text() {
    let encoded = "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx";
    assert_eq!(
        umt_decode_base58_to_string(encoded).unwrap(),
        "The quick brown fox jumps over the lazy dog"
    );
}
