use std::collections::HashMap;
use umt_rust::url::umt_build_url;

fn single(key: &str, value: &str) -> HashMap<String, String> {
    HashMap::from([(key.to_string(), value.to_string())])
}

#[test]
fn test_build_url_parity_encodes_like_url_search_params() {
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("a", "~*-._ !'()"))),
        "https://example.com/?a=%7E*-._+%21%27%28%29"
    );
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("b", "\u{3042}/?&=#+%"))),
        "https://example.com/?b=%E3%81%82%2F%3F%26%3D%23%2B%25"
    );
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("AZaz09", "AZaz09"))),
        "https://example.com/?AZaz09=AZaz09"
    );
}

#[test]
fn test_build_url_parity_keeps_prototype_like_keys() {
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("constructor", "c"))),
        "https://example.com/?constructor=c"
    );
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("prototype", "p"))),
        "https://example.com/?prototype=p"
    );
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("__proto__", "x"))),
        "https://example.com/?__proto__=x"
    );
}

#[test]
fn test_build_url_parity_query_goes_before_fragment() {
    assert_eq!(
        umt_build_url("https://example.com/p#frag", Some(&single("k", "v"))),
        "https://example.com/p?k=v#frag"
    );
    assert_eq!(
        umt_build_url("https://example.com/p?x=1#frag", Some(&single("k", "v"))),
        "https://example.com/p?x=1&k=v#frag"
    );
    assert_eq!(
        umt_build_url("https://example.com#a?b", Some(&single("k", "v"))),
        "https://example.com/?k=v#a?b"
    );
}

#[test]
fn test_build_url_parity_existing_query_without_new_params() {
    assert_eq!(
        umt_build_url("https://example.com/p?x=1", None),
        "https://example.com/p?x=1"
    );
    assert_eq!(
        umt_build_url("https://example.com/p?x=1", Some(&HashMap::new())),
        "https://example.com/p?x=1"
    );
    assert_eq!(
        umt_build_url("https://example.com#f", None),
        "https://example.com/#f"
    );
}

#[test]
fn test_build_url_parity_root_path_is_added_only_for_special_schemes() {
    assert_eq!(
        umt_build_url("ws://example.com", Some(&single("k", "v"))),
        "ws://example.com/?k=v"
    );
    assert_eq!(
        umt_build_url("foo://host", Some(&single("k", "v"))),
        "foo://host?k=v"
    );
    assert_eq!(
        umt_build_url("mailto:user@host", Some(&single("k", "v"))),
        "mailto:user@host?k=v"
    );
    assert_eq!(
        umt_build_url("https://example.com/path", Some(&single("k", "v"))),
        "https://example.com/path?k=v"
    );
}

#[test]
fn test_build_url_parity_empty_key_and_value() {
    assert_eq!(
        umt_build_url("https://example.com", Some(&single("", ""))),
        "https://example.com/?="
    );
}

#[test]
fn test_build_url_parity_multiple_params_are_joined_with_ampersand() {
    let params = HashMap::from([
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);
    let result = umt_build_url("https://example.com", Some(&params));
    assert!(result == "https://example.com/?a=1&b=2" || result == "https://example.com/?b=2&a=1");
}

#[test]
fn test_build_url_parity_existing_query_is_serialized_again_when_params_are_appended() {
    assert_eq!(
        umt_build_url("https://example.com/?a=1%20b", Some(&single("k", "v"))),
        "https://example.com/?a=1+b&k=v"
    );
    assert_eq!(
        umt_build_url("https://example.com/p?x=%7e#frag", Some(&single("k", "v"))),
        "https://example.com/p?x=%7E&k=v#frag"
    );
    assert_eq!(
        umt_build_url("https://e.com/?a=1&&b", Some(&single("k", "v"))),
        "https://e.com/?a=1&b=&k=v"
    );
    assert_eq!(
        umt_build_url("https://e.com/?a=%zz&b=%e9", Some(&single("k", "v"))),
        "https://e.com/?a=%25zz&b=%EF%BF%BD&k=v"
    );
    assert_eq!(
        umt_build_url("https://e.com/?a=b=c&d", Some(&single("k", "v"))),
        "https://e.com/?a=b%3Dc&d=&k=v"
    );
}

#[test]
fn test_build_url_parity_existing_query_is_kept_verbatim_with_empty_params() {
    assert_eq!(
        umt_build_url("https://example.com/?a=1%20b", Some(&HashMap::new())),
        "https://example.com/?a=1%20b"
    );
    assert_eq!(
        umt_build_url("https://example.com/?a=1%20b", None),
        "https://example.com/?a=1%20b"
    );
}
