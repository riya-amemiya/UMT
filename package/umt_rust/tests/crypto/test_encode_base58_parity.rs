use umt_rust::crypto::umt_encode_base58;

#[test]
fn test_encode_base58_parity_long_binary_input() {
    let data: Vec<u8> = (0..64u32)
        .map(|index| ((index * 37 + 11) & 0xFF) as u8)
        .collect();
    assert_eq!(
        umt_encode_base58(&data),
        "DyXZZCSiBREJ8YZ5cULd7PVdKpbkhHvHJ7otbJmLJajJtRsyq1irMMqKimYeKvRmZ8Sc2qWLhKjYR4ekM8RSzkV"
    );
}

#[test]
fn test_encode_base58_parity_only_zero_bytes() {
    assert_eq!(umt_encode_base58([0u8, 0, 0]), "111");
}

#[test]
fn test_encode_base58_parity_leading_zero_bytes() {
    assert_eq!(umt_encode_base58([0u8, 0, 1, 2, 3]), "11Ldp");
}

#[test]
fn test_encode_base58_parity_empty_input() {
    assert_eq!(umt_encode_base58(""), "");
    assert_eq!(umt_encode_base58(Vec::<u8>::new()), "");
}

#[test]
fn test_encode_base58_parity_string_and_bytes_inputs() {
    assert_eq!(
        umt_encode_base58("\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}"),
        "7NAasPYBzpyEe5hmwr1KL"
    );
    assert_eq!(umt_encode_base58([255u8, 254, 253, 252, 251]), "Vt9aq46");
}
