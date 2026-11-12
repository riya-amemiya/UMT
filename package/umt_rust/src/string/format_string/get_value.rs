use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

static ARRAY_INDEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(.+?)\[(-?\d+)\]$").unwrap());

fn property<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Object(map) => map.get(key),
        Value::Array(items) => {
            let is_canonical_index = key == "0"
                || (!key.starts_with('0')
                    && !key.is_empty()
                    && key.bytes().all(|b| b.is_ascii_digit()));
            if is_canonical_index {
                items.get(key.parse::<usize>().ok()?)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Retrieves a value from a JSON object using a dot-notation path with array index support.
///
/// Supports nested properties and array access with positive/negative indices.
/// An array segment also resolves canonical index keys (e.g. "items.0").
///
/// # Arguments
///
/// * `object` - The JSON value to retrieve from
/// * `path` - Dot-notation path (e.g., "user.name", "items[0]", "data[0].items[-1].name")
///
/// # Returns
///
/// The value at the specified path, or None if not found
///
/// # Examples
///
/// ```ignore
/// use serde_json::json;
/// use umt_rust::string::format_string::get_value;
///
/// let obj = json!({"name": "Alice"});
/// assert_eq!(get_value(&obj, "name"), Some(&json!("Alice")));
/// ```
pub fn get_value<'a>(object: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = object;

    for part in path.split('.') {
        let (key, index) = match ARRAY_INDEX_RE.captures(part) {
            Some(caps) => (
                caps.get(1)?.as_str(),
                Some(caps.get(2)?.as_str().parse::<i64>().ok()?),
            ),
            None => (part, None),
        };

        current = property(current, key)?;

        if let Some(index) = index {
            let items = current.as_array()?;
            let resolved = if index < 0 {
                items.len() as i64 + index
            } else {
                index
            };
            current = items.get(usize::try_from(resolved).ok()?)?;
        }
    }

    Some(current)
}
