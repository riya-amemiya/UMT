use super::parse_query_string::decode_percent;
use std::collections::HashMap;

const SPECIAL_SCHEMES: [&str; 6] = ["http", "https", "ws", "wss", "ftp", "file"];
const UPPER_HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// Builds a URL with query parameters appended.
///
/// Keys and values are serialized as `application/x-www-form-urlencoded`,
/// the same way JavaScript `URLSearchParams` does. No key is filtered out.
/// The query string is placed before the fragment of the base URL. When
/// parameters are appended, the existing query string of the base URL is
/// serialized again the same way.
///
/// # Arguments
///
/// * `base` - The base URL string
/// * `params` - An optional map of key-value pairs to append as query parameters
///
/// # Returns
///
/// The complete URL string with query parameters.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use umt_rust::url::umt_build_url;
///
/// let mut params = HashMap::new();
/// params.insert("key".to_string(), "value".to_string());
/// assert_eq!(
///     umt_build_url("https://example.com", Some(&params)),
///     "https://example.com/?key=value"
/// );
/// ```
pub fn umt_build_url(base: &str, params: Option<&HashMap<String, String>>) -> String {
    let (without_fragment, fragment) = match base.find('#') {
        Some(idx) => (&base[..idx], &base[idx..]),
        None => (base, ""),
    };
    let (base_part, existing_query) = match without_fragment.find('?') {
        Some(idx) => (&without_fragment[..idx], &without_fragment[idx + 1..]),
        None => (without_fragment, ""),
    };

    let needs_root_path = base_part.find("://").is_some_and(|authority_start| {
        let scheme = &base_part[..authority_start];
        !base_part[authority_start + 3..].contains('/')
            && SPECIAL_SCHEMES
                .iter()
                .any(|special| scheme.eq_ignore_ascii_case(special))
    });

    let query = match params.filter(|params| !params.is_empty()) {
        Some(params) => {
            let mut query = String::with_capacity(existing_query.len());
            for pair in existing_query.split('&').filter(|pair| !pair.is_empty()) {
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                push_pair(&mut query, &decode_percent(key), &decode_percent(value));
            }
            for (key, value) in params {
                push_pair(&mut query, key, value);
            }
            query
        }
        None => String::from(existing_query),
    };

    let mut result = String::with_capacity(base.len() + query.len() + 2);
    result.push_str(base_part);
    if needs_root_path {
        result.push('/');
    }
    if !query.is_empty() {
        result.push('?');
        result.push_str(&query);
    }
    result.push_str(fragment);
    result
}

/// Appends one `key=value` pair to `query`, separated from the previous pair by `&`.
pub(super) fn push_pair(query: &mut String, key: &str, value: &str) {
    if !query.is_empty() {
        query.push('&');
    }
    encode_component(query, key);
    query.push('=');
    encode_component(query, value);
}

/// Percent-encodes a string for use in URL query parameters and appends it to `output`.
///
/// This follows the `application/x-www-form-urlencoded` convention where
/// spaces are encoded as `+` and other special characters are
/// percent-encoded, matching the behaviour of JavaScript's
/// `URLSearchParams.append`.
fn encode_component(output: &mut String, input: &str) {
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                output.push(char::from(byte));
            }
            b' ' => {
                output.push('+');
            }
            _ => {
                output.push('%');
                output.push(char::from(UPPER_HEX_DIGITS[usize::from(byte >> 4)]));
                output.push(char::from(UPPER_HEX_DIGITS[usize::from(byte & 0x0F)]));
            }
        }
    }
}
