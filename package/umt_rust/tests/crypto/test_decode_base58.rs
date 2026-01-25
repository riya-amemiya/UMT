use umt_rust::crypto::umt_decode_base58;

#[test]
fn test_decodes_a_simple_string() {
    let result = umt_decode_base58("9Ajdvzr").unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Hello");
}

#[test]
fn test_decodes_an_empty_string() {
    let result = umt_decode_base58("").unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_decodes_single_character_strings() {
    assert_eq!(
        String::from_utf8(umt_decode_base58("2g").unwrap()).unwrap(),
        "a"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base58("2h").unwrap()).unwrap(),
        "b"
    );
    assert_eq!(
        String::from_utf8(umt_decode_base58("2i").unwrap()).unwrap(),
        "c"
    );
}

#[test]
fn test_handles_leading_ones_zeros() {
    let result = umt_decode_base58("119Ajdvzr").unwrap();
    assert_eq!(result[0], 0);
    assert_eq!(result[1], 0);
    assert_eq!(String::from_utf8(result[2..].to_vec()).unwrap(), "Hello");
}

#[test]
fn test_throws_error_on_invalid_character_zero() {
    let result = umt_decode_base58("9Ajdvz0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: 0");
}

#[test]
fn test_throws_error_on_invalid_character_uppercase_o() {
    let result = umt_decode_base58("9AjdvzO");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: O");
}

#[test]
fn test_throws_error_on_invalid_character_uppercase_i() {
    let result = umt_decode_base58("9AjdvzI");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: I");
}

#[test]
fn test_throws_error_on_invalid_character_lowercase_l() {
    let result = umt_decode_base58("9Ajdvzl");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Invalid base58 character: l");
}

#[test]
fn test_decodes_longer_text() {
    let encoded = "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx";
    let result = String::from_utf8(umt_decode_base58(encoded).unwrap()).unwrap();
    assert_eq!(result, "The quick brown fox jumps over the lazy dog");
}

#[test]
fn test_decodes_binary_data() {
    let result = umt_decode_base58("Vt9aq46").unwrap();
    assert_eq!(result, vec![255, 254, 253, 252, 251]);
}
