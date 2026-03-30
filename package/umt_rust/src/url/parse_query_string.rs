use std::collections::HashMap;

/// Parses a query string into a key-value map.
///
/// Accepts either a full URL or a raw query string
/// (with or without leading "?").
///
/// Keys that could cause prototype pollution (`__proto__`, `constructor`,
/// `prototype`) are silently dropped for parity with the TypeScript
/// implementation.
///
/// # Arguments
///
/// * `query` - The query string or URL to parse
///
/// # Returns
///
/// A `HashMap<String, String>` of decoded key-value pairs.
///
/// # Example
///
/// ```
/// use umt_rust::url::umt_parse_query_string;
///
/// let result = umt_parse_query_string("?page=1&q=search");
/// assert_eq!(result.get("page").unwrap(), "1");
/// assert_eq!(result.get("q").unwrap(), "search");
/// ```
pub fn umt_parse_query_string(query: &str) -> HashMap<String, String> {
    let search_string = if query.contains("://") {
        // Extract the query portion after '?'
        match query.find('?') {
            Some(idx) => &query[idx..],
            None => return HashMap::new(),
        }
    } else {
        query
    };

    let raw = search_string.strip_prefix('?').unwrap_or(search_string);

    if raw.is_empty() {
        return HashMap::new();
    }

    let mut result = HashMap::new();

    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key_raw, value_raw) = match pair.find('=') {
            Some(idx) => (&pair[..idx], &pair[idx + 1..]),
            None => (pair, ""),
        };

        let key = decode_percent(key_raw);
        let value = decode_percent(value_raw);

        // Prevent prototype pollution by rejecting dangerous keys
        if key == "__proto__" || key == "constructor" || key == "prototype" {
            continue;
        }

        result.insert(key, value);
    }

    result
}

/// Decodes percent-encoded strings, also converting '+' to spaces
/// (application/x-www-form-urlencoded).
fn decode_percent(input: &str) -> String {
    let plus_decoded = input.replace('+', " ");
    let bytes = plus_decoded.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&plus_decoded[i + 1..i + 3], 16)
        {
            result.push(byte);
            i += 3;
            continue;
        }
        result.push(bytes[i]);
        i += 1;
    }

    String::from_utf8_lossy(&result).into_owned()
}
