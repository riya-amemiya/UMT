//! Dictionary object type checking utility

use std::collections::HashMap;

/// Determines if the value is a dictionary-type object (HashMap)
///
/// In Rust, this checks if the input is a valid HashMap reference.
/// Since Rust is strongly typed, this always returns true for HashMap inputs.
///
/// # Arguments
/// * `_object` - The HashMap to check
///
/// # Returns
/// Always returns true since the input is typed as HashMap
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_dictionary_object;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("key", "value");
/// assert!(umt_is_dictionary_object(&map));
/// ```
#[inline]
pub fn umt_is_dictionary_object<K, V>(_object: &HashMap<K, V>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dictionary_object() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("a", 1);
        assert!(umt_is_dictionary_object(&map));

        let empty: HashMap<String, String> = HashMap::new();
        assert!(umt_is_dictionary_object(&empty));
    }
}
