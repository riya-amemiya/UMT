use std::collections::HashMap;

/// Parses a query string into a key-value map.
///
/// Accepts either a full URL or a raw query string
/// (with or without leading "?").
///
/// Keys that could cause prototype pollution in downstream
/// JavaScript consumers (`__proto__`, `constructor`, `prototype`)
/// are silently dropped for defence-in-depth.
///
/// # Arguments
///
/// * `query` - The query string or URL to parse
///
/// # Returns
///
/// A `HashMap<String, String>` of key-value pairs.
///
/// # Example
///
/// ```
/// use umt_rust::url::umt_parse_query_string;
///
/// let result = umt_parse_query_string("?page=1&q=search");
/// assert_eq!(result.get("page").unwrap(), "1");
/// assert_eq!(result.get("q").unwrap(), "search");
///
/// let result = umt_parse_query_string("foo=bar&baz=qux");
/// assert_eq!(result.get("foo").unwrap(), "bar");
/// assert_eq!(result.get("baz").unwrap(), "qux");
///
/// let result = umt_parse_query_string("https://example.com?a=1&b=2");
/// assert_eq!(result.get("a").unwrap(), "1");
/// assert_eq!(result.get("b").unwrap(), "2");
/// ```
pub fn umt_parse_query_string(query: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    // Extract the query portion from a full URL or raw query string
    let search_string = if query.contains("://") {
        match query.find('?') {
            Some(pos) => &query[pos + 1..],
            None => return result,
        }
    } else if let Some(stripped) = query.strip_prefix('?') {
        stripped
    } else {
        query
    };

    if search_string.is_empty() {
        return result;
    }

    for pair in search_string.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key_raw, value_raw) = match pair.find('=') {
            Some(pos) => (&pair[..pos], &pair[pos + 1..]),
            None => (pair, ""),
        };

        let key = url_decode(key_raw);
        if key == "__proto__" || key == "constructor" || key == "prototype" {
            continue;
        }

        let value = url_decode(value_raw);
        result.insert(key, value);
    }

    result
}

/// Decodes a percent-encoded string and converts '+' to spaces,
/// matching the behaviour of `URLSearchParams`.
fn url_decode(input: &str) -> String {
    let plus_replaced = input.replace('+', " ");
    let bytes = plus_replaced.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&plus_replaced[i + 1..i + 3], 16)
        {
            decoded.push(byte);
            i += 3;
            continue;
        }
        decoded.push(bytes[i]);
        i += 1;
    }

    String::from_utf8_lossy(&decoded).into_owned()
}
