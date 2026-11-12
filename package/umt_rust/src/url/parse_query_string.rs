use std::collections::HashMap;

/// Parses a query string into a key-value map.
///
/// Accepts either a full URL or a raw query string
/// (with or without leading "?"). The fragment of a full URL is ignored.
///
/// When a key appears more than once, the last value wins. No key is
/// filtered out.
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
        let without_fragment = query.find('#').map_or(query, |idx| &query[..idx]);
        match without_fragment.find('?') {
            Some(idx) => &without_fragment[idx..],
            None => return HashMap::new(),
        }
    } else {
        query
    };

    let raw = search_string.strip_prefix('?').unwrap_or(search_string);

    let mut result = HashMap::new();

    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key_raw, value_raw) = match pair.find('=') {
            Some(idx) => (&pair[..idx], &pair[idx + 1..]),
            None => (pair, ""),
        };

        result.insert(decode_percent(key_raw), decode_percent(value_raw));
    }

    result
}

/// Reads the value of an ASCII hexadecimal digit.
fn hex_value(byte: Option<&u8>) -> Option<u8> {
    byte.and_then(|&digit| char::from(digit).to_digit(16))
        .map(|value| value as u8)
}

/// Decodes percent-encoded strings, also converting '+' to spaces
/// (application/x-www-form-urlencoded).
pub(super) fn decode_percent(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        let decoded = if bytes[i] == b'%' {
            hex_value(bytes.get(i + 1)).zip(hex_value(bytes.get(i + 2)))
        } else {
            None
        };
        match decoded {
            Some((high, low)) => {
                result.push((high << 4) | low);
                i += 3;
            }
            None => {
                result.push(if bytes[i] == b'+' { b' ' } else { bytes[i] });
                i += 1;
            }
        }
    }

    String::from_utf8_lossy(&result).into_owned()
}
