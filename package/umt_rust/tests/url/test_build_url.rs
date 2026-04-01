use std::collections::HashMap;
use umt_rust::url::umt_build_url;

#[test]
fn test_build_url_with_query_parameters() {
    let mut params = HashMap::new();
    params.insert("page".to_string(), "1".to_string());
    params.insert("q".to_string(), "search".to_string());
    let result = umt_build_url("https://example.com", Some(&params));
    assert!(result.contains("page=1"));
    assert!(result.contains("q=search"));
    assert!(result.starts_with("https://example.com/?"));
}

#[test]
fn test_returns_base_url_when_no_params() {
    let result = umt_build_url("https://example.com/path", None);
    assert_eq!(result, "https://example.com/path");
}

#[test]
fn test_returns_base_url_with_empty_params() {
    let params = HashMap::new();
    let result = umt_build_url("https://example.com/path", Some(&params));
    assert_eq!(result, "https://example.com/path");
}

#[test]
fn test_encodes_special_characters_in_values() {
    let mut params = HashMap::new();
    params.insert("q".to_string(), "hello world".to_string());
    let result = umt_build_url("https://example.com", Some(&params));
    assert!(result.contains("q=hello+world"));
}

#[test]
fn test_handles_existing_query_parameters_in_base() {
    let mut params = HashMap::new();
    params.insert("added".to_string(), "2".to_string());
    let result = umt_build_url("https://example.com?existing=1", Some(&params));
    assert!(result.contains("existing=1"));
    assert!(result.contains("added=2"));
}

#[test]
fn test_handles_single_parameter() {
    let mut params = HashMap::new();
    params.insert("key".to_string(), "value".to_string());
    let result = umt_build_url("https://example.com", Some(&params));
    assert_eq!(result, "https://example.com/?key=value");
}

#[test]
fn test_skips_prototype_pollution_keys() {
    let mut params = HashMap::new();
    params.insert("__proto__".to_string(), "evil".to_string());
    params.insert("constructor".to_string(), "evil".to_string());
    params.insert("prototype".to_string(), "evil".to_string());
    params.insert("safe".to_string(), "value".to_string());
    let result = umt_build_url("https://example.com", Some(&params));
    assert_eq!(result, "https://example.com/?safe=value");
    assert!(!result.contains("__proto__"));
    assert!(!result.contains("constructor"));
    assert!(!result.contains("prototype"));
}

#[test]
fn test_normalizes_base_url_with_trailing_slash() {
    let result = umt_build_url("https://example.com", None);
    assert_eq!(result, "https://example.com/");
}
