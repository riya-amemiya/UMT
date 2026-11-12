use umt_rust::string::umt_sentence_case;

#[test]
fn test_sentence_case_doc_examples() {
    assert_eq!(
        umt_sentence_case("helloWorld foo-bar"),
        "Hello world foo bar"
    );
    assert_eq!(umt_sentence_case("XMLHttpRequest"), "Xml http request");
    assert_eq!(umt_sentence_case(""), "");
}

#[test]
fn test_sentence_case_lower_cases_rest() {
    assert_eq!(umt_sentence_case("HELLO WORLD"), "Hello world");
    assert_eq!(umt_sentence_case("snake_case_value"), "Snake case value");
}

#[test]
fn test_sentence_case_single_word() {
    assert_eq!(umt_sentence_case("hello"), "Hello");
}

#[test]
fn test_sentence_case_no_words() {
    assert_eq!(umt_sentence_case("  --  "), "");
}

#[test]
fn test_sentence_case_non_ascii_letters() {
    assert_eq!(
        umt_sentence_case("\u{00C9}CLAIR au CHOCOLAT"),
        "\u{00C9}clair au chocolat"
    );
}
