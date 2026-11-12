use std::collections::HashMap;
use umt_rust::url::umt_parse_query_string;

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[test]
fn test_parse_query_string_parity_keeps_prototype_like_keys() {
    assert_eq!(
        umt_parse_query_string("?__proto__=1&constructor=2&prototype=3&ok=4"),
        map(&[
            ("__proto__", "1"),
            ("constructor", "2"),
            ("prototype", "3"),
            ("ok", "4")
        ])
    );
}

#[test]
fn test_parse_query_string_parity_last_duplicate_wins() {
    assert_eq!(umt_parse_query_string("a=1&a=2"), map(&[("a", "2")]));
}

#[test]
fn test_parse_query_string_parity_pairs_without_key_or_value() {
    assert_eq!(umt_parse_query_string("a"), map(&[("a", "")]));
    assert_eq!(umt_parse_query_string("=v"), map(&[("", "v")]));
    assert_eq!(umt_parse_query_string("a=b=c"), map(&[("a", "b=c")]));
    assert_eq!(umt_parse_query_string("&&a=1&&"), map(&[("a", "1")]));
}

#[test]
fn test_parse_query_string_parity_percent_and_plus_decoding() {
    assert_eq!(
        umt_parse_query_string("a=%E3%81%82"),
        map(&[("a", "\u{3042}")])
    );
    assert_eq!(umt_parse_query_string("a+b=c+d"), map(&[("a b", "c d")]));
    assert_eq!(umt_parse_query_string("a=%2B"), map(&[("a", "+")]));
    assert_eq!(umt_parse_query_string("a=%4a%4A"), map(&[("a", "JJ")]));
}

#[test]
fn test_parse_query_string_parity_malformed_percent_sequences_stay_literal() {
    assert_eq!(umt_parse_query_string("a=%ZZ"), map(&[("a", "%ZZ")]));
    assert_eq!(umt_parse_query_string("a=%"), map(&[("a", "%")]));
    assert_eq!(umt_parse_query_string("a=%4"), map(&[("a", "%4")]));
    assert_eq!(umt_parse_query_string("a=%4Z"), map(&[("a", "%4Z")]));
    assert_eq!(
        umt_parse_query_string("a=%a\u{00E9}"),
        map(&[("a", "%a\u{00E9}")])
    );
}

#[test]
fn test_parse_query_string_parity_invalid_utf8_becomes_replacement_character() {
    assert_eq!(umt_parse_query_string("a=%ff"), map(&[("a", "\u{FFFD}")]));
}

#[test]
fn test_parse_query_string_parity_only_one_leading_question_mark_is_removed() {
    assert_eq!(umt_parse_query_string("??a=1"), map(&[("?a", "1")]));
}

#[test]
fn test_parse_query_string_parity_full_url_ignores_fragment() {
    assert_eq!(
        umt_parse_query_string("https://e.com?a=1#frag"),
        map(&[("a", "1")])
    );
    assert_eq!(
        umt_parse_query_string("https://e.com/#/route?a=1"),
        HashMap::new()
    );
    assert_eq!(umt_parse_query_string("https://e.com/path"), HashMap::new());
    assert_eq!(
        umt_parse_query_string("https://e.com/path?x=1"),
        map(&[("x", "1")])
    );
}

#[test]
fn test_parse_query_string_parity_empty_inputs() {
    assert_eq!(umt_parse_query_string(""), HashMap::new());
    assert_eq!(umt_parse_query_string("?"), HashMap::new());
}
