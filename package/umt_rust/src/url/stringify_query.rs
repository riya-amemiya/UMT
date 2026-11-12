use super::build_url::push_pair;
use std::collections::HashMap;

/// Serializes a key-value map into a query string.
///
/// The inverse of `umt_parse_query_string`. Keys and values are serialized
/// as `application/x-www-form-urlencoded`, the same way `umt_build_url`
/// appends parameters. The result has no leading "?". Pairs follow the
/// iteration order of the map.
///
/// # Arguments
///
/// * `params` - A map of key-value pairs to serialize
///
/// # Returns
///
/// The query string, or an empty string for an empty map.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use umt_rust::url::umt_stringify_query;
///
/// let mut params = HashMap::new();
/// params.insert("q".to_string(), "hello world".to_string());
/// assert_eq!(umt_stringify_query(&params), "q=hello+world");
/// ```
pub fn umt_stringify_query(params: &HashMap<String, String>) -> String {
    let mut query = String::new();
    for (key, value) in params {
        push_pair(&mut query, key, value);
    }
    query
}
