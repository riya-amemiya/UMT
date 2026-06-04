use umt_rust::string::{MaskOptions, umt_mask};

#[test]
fn test_masks_the_middle_with_default_options() {
    assert_eq!(umt_mask("secret", &MaskOptions::default()), "s****t");
}

#[test]
fn test_masks_with_custom_start_and_end() {
    assert_eq!(
        umt_mask(
            "1234567890",
            &MaskOptions {
                start: Some(2),
                end: Some(4),
                char: None,
            }
        ),
        "12****7890"
    );
}

#[test]
fn test_uses_custom_mask_character() {
    assert_eq!(
        umt_mask(
            "secret",
            &MaskOptions {
                start: None,
                end: None,
                char: Some("#".to_string()),
            }
        ),
        "s####t"
    );
}

#[test]
fn test_returns_input_unchanged_when_start_end_equals_length() {
    assert_eq!(
        umt_mask(
            "abc",
            &MaskOptions {
                start: Some(2),
                end: Some(1),
                char: None,
            }
        ),
        "abc"
    );
}

#[test]
fn test_returns_input_unchanged_when_start_end_exceeds_length() {
    assert_eq!(
        umt_mask(
            "abc",
            &MaskOptions {
                start: Some(5),
                end: Some(5),
                char: None,
            }
        ),
        "abc"
    );
}

#[test]
fn test_handles_surrogate_pairs_as_single_graphemes() {
    let input = "\u{1F600}abcde\u{1F600}";
    let expected = "\u{1F600}*****\u{1F600}";
    assert_eq!(
        umt_mask(
            input,
            &MaskOptions {
                start: Some(1),
                end: Some(1),
                char: None,
            }
        ),
        expected
    );
}

#[test]
fn test_returns_input_unchanged_for_empty_string() {
    assert_eq!(umt_mask("", &MaskOptions::default()), "");
}
