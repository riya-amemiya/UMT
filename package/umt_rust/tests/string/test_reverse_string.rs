use umt_rust::string::umt_reverse_string;

#[test]
fn test_reverse_a_three_character_string() {
    assert_eq!(umt_reverse_string("abc"), "cba");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_reverse_string(""), "");
}

#[test]
fn test_handle_single_character() {
    assert_eq!(umt_reverse_string("a"), "a");
}

#[test]
fn test_reverse_a_two_character_string() {
    assert_eq!(umt_reverse_string("ab"), "ba");
}

#[test]
fn test_reverse_a_four_character_string() {
    assert_eq!(umt_reverse_string("abcd"), "dcba");
}

#[test]
fn test_reverse_a_five_character_string() {
    assert_eq!(umt_reverse_string("abcde"), "edcba");
}

#[test]
fn test_reverse_a_six_character_string() {
    assert_eq!(umt_reverse_string("abcdef"), "fedcba");
}

#[test]
fn test_reverse_a_seven_character_string() {
    assert_eq!(umt_reverse_string("abcdefg"), "gfedcba");
}

#[test]
fn test_reverse_an_eight_character_string() {
    assert_eq!(umt_reverse_string("abcdefgh"), "hgfedcba");
}

#[test]
fn test_handle_whitespace() {
    assert_eq!(umt_reverse_string("hello world"), "dlrow olleh");
    assert_eq!(umt_reverse_string("  "), "  ");
}

#[test]
fn test_handle_special_characters() {
    assert_eq!(umt_reverse_string("!@#$%"), "%$#@!");
    assert_eq!(umt_reverse_string("a-b-c"), "c-b-a");
}

#[test]
fn test_handle_numbers_in_string() {
    assert_eq!(umt_reverse_string("12345"), "54321");
    assert_eq!(umt_reverse_string("a1b2c3"), "3c2b1a");
}

#[test]
fn test_handle_unicode_characters() {
    assert_eq!(umt_reverse_string("あいう"), "ういあ");
    assert_eq!(umt_reverse_string("日本語"), "語本日");
}
