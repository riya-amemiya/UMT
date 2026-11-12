use std::collections::HashMap;
use umt_rust::url::{umt_build_url, umt_parse_query_string, umt_stringify_query};

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[test]
fn test_serializes_a_single_pair() {
    assert_eq!(umt_stringify_query(&map(&[("page", "1")])), "page=1");
}

#[test]
fn test_joins_pairs_with_ampersand() {
    let query = umt_stringify_query(&map(&[("page", "1"), ("q", "search")]));
    assert!(
        query == "page=1&q=search" || query == "q=search&page=1",
        "{}",
        query
    );
}

#[test]
fn test_empty_map() {
    assert_eq!(umt_stringify_query(&HashMap::new()), "");
}

#[test]
fn test_form_urlencoded() {
    assert_eq!(
        umt_stringify_query(&map(&[("q", "hello world")])),
        "q=hello+world"
    );
    assert_eq!(umt_stringify_query(&map(&[("t", "a&b=c")])), "t=a%26b%3Dc");
    assert_eq!(
        umt_stringify_query(&map(&[("s", "~*-._!'()")])),
        "s=%7E*-._%21%27%28%29"
    );
    assert_eq!(
        umt_stringify_query(&map(&[("u", "\u{3042}")])),
        "u=%E3%81%82"
    );
    assert_eq!(umt_stringify_query(&map(&[("e", "")])), "e=");
}

#[test]
fn test_encodes_keys() {
    assert_eq!(umt_stringify_query(&map(&[("a b", "1")])), "a+b=1");
    assert_eq!(umt_stringify_query(&map(&[("c&d", "2")])), "c%26d=2");
}

#[test]
fn test_matches_build_url() {
    let params = map(&[("q", "hello world")]);
    assert_eq!(
        format!("https://example.com/?{}", umt_stringify_query(&params)),
        umt_build_url("https://example.com", Some(&params))
    );
    let params = map(&[("emoji", "\u{1F600}")]);
    assert_eq!(
        format!("https://example.com/?{}", umt_stringify_query(&params)),
        umt_build_url("https://example.com", Some(&params))
    );
}

#[test]
fn test_inverse_of_parse_query_string() {
    let params = map(&[("q", "hello world"), ("tag", "a&b=c"), ("plus", "1+1")]);
    assert_eq!(
        umt_parse_query_string(&umt_stringify_query(&params)),
        params
    );
}
