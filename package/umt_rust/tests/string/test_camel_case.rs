use umt_rust::string::umt_camel_case;

#[test]
fn test_convert_kebab_case_to_camel_case() {
    assert_eq!(umt_camel_case("hello-world"), "helloWorld");
    assert_eq!(umt_camel_case("foo-bar-baz"), "fooBarBaz");
}

#[test]
fn test_convert_snake_case_to_camel_case() {
    assert_eq!(umt_camel_case("hello_world"), "helloWorld");
    assert_eq!(umt_camel_case("foo_bar_baz"), "fooBarBaz");
}

#[test]
fn test_convert_space_separated_words_to_camel_case() {
    assert_eq!(umt_camel_case("hello world"), "helloWorld");
    assert_eq!(umt_camel_case("foo bar baz"), "fooBarBaz");
}

#[test]
fn test_handle_pascal_case_input() {
    assert_eq!(umt_camel_case("HelloWorld"), "helloWorld");
    assert_eq!(umt_camel_case("FooBarBaz"), "fooBarBaz");
}

#[test]
fn test_handle_already_camel_case_input() {
    assert_eq!(umt_camel_case("helloWorld"), "helloWorld");
    assert_eq!(umt_camel_case("fooBarBaz"), "fooBarBaz");
}

#[test]
fn test_handle_mixed_separators() {
    assert_eq!(umt_camel_case("hello-world_test case"), "helloWorldTestCase");
    assert_eq!(umt_camel_case("foo_bar-baz qux"), "fooBarBazQux");
}

#[test]
fn test_handle_numbers() {
    assert_eq!(umt_camel_case("hello-world-2"), "helloWorld2");
    assert_eq!(umt_camel_case("test_case_123"), "testCase123");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_camel_case(""), "");
}

#[test]
fn test_handle_single_word() {
    assert_eq!(umt_camel_case("hello"), "hello");
    assert_eq!(umt_camel_case("HELLO"), "hELLO");
}

#[test]
fn test_handle_special_characters() {
    assert_eq!(umt_camel_case("hello@world"), "helloWorld");
    assert_eq!(umt_camel_case("foo#bar$baz"), "fooBarBaz");
}

#[test]
fn test_handle_leading_trailing_separators() {
    assert_eq!(umt_camel_case("-hello-world-"), "helloWorld");
    assert_eq!(umt_camel_case("_foo_bar_"), "fooBar");
}

#[test]
fn test_handle_multiple_consecutive_separators() {
    assert_eq!(umt_camel_case("hello---world"), "helloWorld");
    assert_eq!(umt_camel_case("foo___bar"), "fooBar");
}
