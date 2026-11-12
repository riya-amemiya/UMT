use umt_rust::crypto::umt_decode_base58;

#[test]
fn test_decode_base58_parity_long_binary_input() {
    let expected: Vec<u8> = (0..64u32)
        .map(|index| ((index * 37 + 11) & 0xFF) as u8)
        .collect();
    assert_eq!(
        umt_decode_base58(
            "DyXZZCSiBREJ8YZ5cULd7PVdKpbkhHvHJ7otbJmLJajJtRsyq1irMMqKimYeKvRmZ8Sc2qWLhKjYR4ekM8RSzkV"
        )
        .unwrap(),
        expected
    );
}

#[test]
fn test_decode_base58_parity_only_ones() {
    assert_eq!(umt_decode_base58("111").unwrap(), vec![0, 0, 0]);
}

#[test]
fn test_decode_base58_parity_leading_ones() {
    assert_eq!(umt_decode_base58("1112").unwrap(), vec![0, 0, 0, 1]);
    assert_eq!(umt_decode_base58("11Ldp").unwrap(), vec![0, 0, 1, 2, 3]);
}

#[test]
fn test_decode_base58_parity_empty_input() {
    assert_eq!(umt_decode_base58("").unwrap(), Vec::<u8>::new());
}

#[test]
fn test_decode_base58_parity_highest_digit() {
    assert_eq!(
        umt_decode_base58("zzzzzzzzzzzz").unwrap(),
        vec![78, 144, 10, 187, 83, 230, 183, 15, 255]
    );
}

#[test]
fn test_decode_base58_parity_invalid_ascii_characters_are_errors() {
    for value in ["0", "O", "I", "l", "9Ajdvzr!"] {
        let error = umt_decode_base58(value).unwrap_err();
        assert!(error.message.starts_with("Invalid base58 character: "));
    }
    assert_eq!(
        umt_decode_base58("9A0").unwrap_err().message,
        "Invalid base58 character: 0"
    );
}

#[test]
fn test_decode_base58_parity_non_ascii_characters_are_errors() {
    assert_eq!(
        umt_decode_base58("\u{00E9}").unwrap_err().message,
        "Invalid base58 character: \u{00E9}"
    );
    assert_eq!(
        umt_decode_base58("9A\u{3042}").unwrap_err().message,
        "Invalid base58 character: \u{3042}"
    );
}
