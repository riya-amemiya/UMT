use umt_rust::string::umt_kebab_case;

#[test]
fn test_convert_camel_case_to_kebab_case() {
    assert_eq!(umt_kebab_case("helloWorld"), "hello-world");
    assert_eq!(umt_kebab_case("fooBarBaz"), "foo-bar-baz");
}

#[test]
fn test_convert_pascal_case_to_kebab_case() {
    assert_eq!(umt_kebab_case("HelloWorld"), "hello-world");
    assert_eq!(umt_kebab_case("FooBarBaz"), "foo-bar-baz");
}

#[test]
fn test_convert_snake_case_to_kebab_case() {
    assert_eq!(umt_kebab_case("hello_world"), "hello-world");
    assert_eq!(umt_kebab_case("foo_bar_baz"), "foo-bar-baz");
}

#[test]
fn test_convert_space_separated_words_to_kebab_case() {
    assert_eq!(umt_kebab_case("hello world"), "hello-world");
    assert_eq!(umt_kebab_case("foo bar baz"), "foo-bar-baz");
}

#[test]
fn test_handle_already_kebab_case_input() {
    assert_eq!(umt_kebab_case("hello-world"), "hello-world");
    assert_eq!(umt_kebab_case("foo-bar-baz"), "foo-bar-baz");
}

#[test]
fn test_handle_mixed_separators() {
    assert_eq!(
        umt_kebab_case("helloWorld_test case"),
        "hello-world-test-case"
    );
    assert_eq!(umt_kebab_case("fooBar-baz qux"), "foo-bar-baz-qux");
}

#[test]
fn test_handle_numbers() {
    assert_eq!(umt_kebab_case("helloWorld2"), "hello-world2");
    assert_eq!(umt_kebab_case("testCase123"), "test-case123");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_kebab_case(""), "");
}

#[test]
fn test_handle_single_word() {
    assert_eq!(umt_kebab_case("hello"), "hello");
    assert_eq!(umt_kebab_case("HELLO"), "hello");
}

#[test]
fn test_remove_special_characters() {
    assert_eq!(umt_kebab_case("hello@world"), "hello-world");
    assert_eq!(umt_kebab_case("foo#bar$baz"), "foo-bar-baz");
}

#[test]
fn test_handle_leading_trailing_separators() {
    assert_eq!(umt_kebab_case("-hello-world-"), "hello-world");
    assert_eq!(umt_kebab_case("_foo_bar_"), "foo-bar");
}

#[test]
fn test_handle_multiple_consecutive_separators() {
    assert_eq!(umt_kebab_case("hello---world"), "hello-world");
    assert_eq!(umt_kebab_case("foo___bar"), "foo-bar");
}

#[test]
fn test_handle_complex_mixed_case() {
    assert_eq!(umt_kebab_case("XMLHttpRequest"), "xml-http-request");
    assert_eq!(umt_kebab_case("getElementById"), "get-element-by-id");
}

#[test]
fn test_handle_acronyms() {
    assert_eq!(umt_kebab_case("HTML"), "html");
    assert_eq!(umt_kebab_case("XMLParser"), "xml-parser");
}

#[test]
fn test_non_ascii_characters_become_dashes() {
    assert_eq!(umt_kebab_case("h\u{e9}llo"), "h-llo");
    assert_eq!(umt_kebab_case("a\u{e9}B"), "a-b");
    assert_eq!(umt_kebab_case("\u{65e5}\u{672c}\u{8a9e}"), "");
}

#[test]
fn test_digits_do_not_create_boundaries() {
    assert_eq!(umt_kebab_case("AB1"), "ab1");
    assert_eq!(umt_kebab_case("a1B"), "a1b");
    assert_eq!(umt_kebab_case("version2Beta"), "version2beta");
    assert_eq!(umt_kebab_case("HTML5Parser"), "html5parser");
}

#[test]
fn test_uppercase_run_boundaries() {
    assert_eq!(umt_kebab_case("A"), "a");
    assert_eq!(umt_kebab_case("Ab"), "ab");
    assert_eq!(umt_kebab_case("ABC"), "abc");
    assert_eq!(umt_kebab_case("ABc"), "a-bc");
    assert_eq!(umt_kebab_case("ABCd"), "ab-cd");
    assert_eq!(umt_kebab_case("aBC"), "a-bc");
    assert_eq!(umt_kebab_case("getHTTPResponse"), "get-http-response");
}

#[test]
fn test_separator_runs_collapse_and_edges_are_trimmed() {
    assert_eq!(umt_kebab_case("--a--b--"), "a-b");
    assert_eq!(umt_kebab_case("aB-C"), "a-b-c");
    assert_eq!(umt_kebab_case("fooBar_baz qux"), "foo-bar-baz-qux");
    assert_eq!(umt_kebab_case("-"), "");
    assert_eq!(umt_kebab_case(""), "");
}
