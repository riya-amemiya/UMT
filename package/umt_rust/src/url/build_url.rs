use std::collections::HashMap;

/// Builds a URL with query parameters appended.
///
/// Dangerous keys (`__proto__`, `constructor`, `prototype`) are silently
/// dropped to prevent prototype-pollution-style attacks, matching the
/// TypeScript source-of-truth behaviour.
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
    let empty = HashMap::new();
    let params = params.unwrap_or(&empty);

    // Separate the base into the part before '?' and any existing query string
    let (base_part, existing_query) = match base.find('?') {
        Some(idx) => (&base[..idx], &base[idx + 1..]),
        None => (base, ""),
    };

    // Determine if base has a scheme with authority (e.g. "https://host")
    // and if the path portion is empty, normalize it to "/"
    let normalized_base = if let Some(authority_start) = base_part.find("://") {
        let after_scheme = &base_part[authority_start + 3..];
        if !after_scheme.contains('/') {
            // No path after authority — add trailing slash
            format!("{base_part}/")
        } else {
            base_part.to_string()
        }
    } else {
        base_part.to_string()
    };

    // Build safe params (skip dangerous keys)
    let safe_params: Vec<(&str, &str)> = params
        .iter()
        .filter(|(k, _)| {
            let k = k.as_str();
            k != "__proto__" && k != "constructor" && k != "prototype"
        })
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    // Encode the new query string
    let new_query = safe_params
        .iter()
        .map(|(k, v)| format!("{}={}", encode_component(k), encode_component(v)))
        .collect::<Vec<_>>()
        .join("&");

    // Combine existing and new query strings
    let combined_query = match (existing_query.is_empty(), new_query.is_empty()) {
        (false, false) => format!("{existing_query}&{new_query}"),
        (true, false) => new_query,
        (false, true) => existing_query.to_string(),
        (true, true) => String::new(),
    };

    if combined_query.is_empty() {
        normalized_base
    } else {
        format!("{normalized_base}?{combined_query}")
    }
}

/// Percent-encodes a string for use in URL query parameters.
///
/// This follows the `application/x-www-form-urlencoded` convention where
/// spaces are encoded as `+` and other special characters are
/// percent-encoded, matching the behaviour of JavaScript's
/// `URLSearchParams.append`.
fn encode_component(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                result.push(byte as char);
            }
            b' ' => {
                result.push('+');
            }
            _ => {
                result.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    result
}
