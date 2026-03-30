use umt_rust::url::umt_parse_query_string;

#[test]
fn test_parse_query_string_with_leading_question_mark() {
    let result = umt_parse_query_string("?page=1&q=search");
    assert_eq!(result.get("page").unwrap(), "1");
    assert_eq!(result.get("q").unwrap(), "search");
    assert_eq!(result.len(), 2);
}

#[test]
fn test_parse_query_string_without_leading_question_mark() {
    let result = umt_parse_query_string("foo=bar&baz=qux");
    assert_eq!(result.get("foo").unwrap(), "bar");
    assert_eq!(result.get("baz").unwrap(), "qux");
    assert_eq!(result.len(), 2);
}

#[test]
fn test_parse_full_url() {
    let result = umt_parse_query_string("https://example.com?a=1&b=2");
    assert_eq!(result.get("a").unwrap(), "1");
    assert_eq!(result.get("b").unwrap(), "2");
    assert_eq!(result.len(), 2);
}

#[test]
fn test_parse_empty_string() {
    let result = umt_parse_query_string("");
    assert!(result.is_empty());
}

#[test]
fn test_parse_question_mark_only() {
    let result = umt_parse_query_string("?");
    assert!(result.is_empty());
}

#[test]
fn test_parse_encoded_values() {
    let result = umt_parse_query_string("?q=hello%20world");
    assert_eq!(result.get("q").unwrap(), "hello world");
}

#[test]
fn test_parse_single_parameter() {
    let result = umt_parse_query_string("?key=value");
    assert_eq!(result.get("key").unwrap(), "value");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_parse_url_with_no_query_string() {
    let result = umt_parse_query_string("https://example.com/path");
    assert!(result.is_empty());
}

#[test]
fn test_reject_proto_key() {
    let result = umt_parse_query_string("?__proto__=polluted&safe=value");
    assert_eq!(result.get("safe").unwrap(), "value");
    assert!(!result.contains_key("__proto__"));
    assert_eq!(result.len(), 1);
}

#[test]
fn test_reject_constructor_and_prototype_keys() {
    let result = umt_parse_query_string("?constructor=bad&prototype=bad&ok=good");
    assert_eq!(result.get("ok").unwrap(), "good");
    assert!(!result.contains_key("constructor"));
    assert!(!result.contains_key("prototype"));
    assert_eq!(result.len(), 1);
}
