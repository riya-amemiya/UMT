use umt_rust::string::umt_constant_case;

#[test]
fn test_convert_camel_case_to_constant_case() {
    assert_eq!(umt_constant_case("helloWorld"), "HELLO_WORLD");
}

#[test]
fn test_convert_space_separated_words() {
    assert_eq!(umt_constant_case("Hello World"), "HELLO_WORLD");
}

#[test]
fn test_convert_kebab_case() {
    assert_eq!(umt_constant_case("foo-bar-baz"), "FOO_BAR_BAZ");
}

#[test]
fn test_handle_single_word() {
    assert_eq!(umt_constant_case("hello"), "HELLO");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_constant_case(""), "");
}
