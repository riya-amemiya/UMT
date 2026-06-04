use umt_rust::string::umt_snake_case;

#[test]
fn test_converts_camel_case() {
    assert_eq!(umt_snake_case("helloWorld"), "hello_world");
}

#[test]
fn test_converts_space_separated_words() {
    assert_eq!(umt_snake_case("Hello World"), "hello_world");
}

#[test]
fn test_converts_kebab_case() {
    assert_eq!(umt_snake_case("foo-bar-baz"), "foo_bar_baz");
}

#[test]
fn test_converts_pascal_case() {
    assert_eq!(umt_snake_case("FooBar"), "foo_bar");
}

#[test]
fn test_returns_empty_for_empty_string() {
    assert_eq!(umt_snake_case(""), "");
}

#[test]
fn test_handles_acronyms() {
    assert_eq!(umt_snake_case("XMLHttpRequest"), "xml_http_request");
}
