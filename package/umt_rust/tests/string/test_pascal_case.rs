use umt_rust::string::umt_pascal_case;

#[test]
fn test_converts_kebab_case() {
    assert_eq!(umt_pascal_case("hello-world"), "HelloWorld");
}

#[test]
fn test_converts_snake_case() {
    assert_eq!(umt_pascal_case("hello_world"), "HelloWorld");
}

#[test]
fn test_converts_space_separated_words() {
    assert_eq!(umt_pascal_case("hello world"), "HelloWorld");
}

#[test]
fn test_converts_camel_case_to_pascal_case() {
    assert_eq!(umt_pascal_case("helloWorld"), "HelloWorld");
}

#[test]
fn test_returns_empty_for_empty_string() {
    assert_eq!(umt_pascal_case(""), "");
}

#[test]
fn test_handles_acronyms() {
    assert_eq!(umt_pascal_case("XMLHttpRequest"), "XmlHttpRequest");
}
