//! Map validation module
//! Provides validation for `HashMap` instances, optionally delegating to
//! per-entry validators for keys and values, mirroring how `umt_validate_array`
//! validates each element of an array.

use std::collections::HashMap;
use std::hash::Hash;

use super::core::ValidateCoreReturnType;

/// Validates a `HashMap` with optional per-entry key and value validators
///
/// The entry validators are applied to every entry of the map and return a
/// `ValidateCoreReturnType` so that, on the first failure, the failing entry's
/// own message is surfaced, mirroring the TypeScript `map()` validator.
///
/// # Arguments
/// * `value` - The map to validate
/// * `key_validator` - Optional validator applied to every key
/// * `value_validator` - Optional validator applied to every value
/// * `message` - Custom error message for map validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::map::umt_validate_map;
/// use std::collections::HashMap;
///
/// let mut m: HashMap<String, i32> = HashMap::new();
/// m.insert("a".to_string(), 1);
/// let result = umt_validate_map(
///     &m,
///     None::<fn(&String) -> umt_rust::validate::core::ValidateCoreReturnType<String>>,
///     None::<fn(&i32) -> umt_rust::validate::core::ValidateCoreReturnType<i32>>,
///     None,
/// );
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_map<K, V, KF, VF>(
    value: &HashMap<K, V>,
    key_validator: Option<KF>,
    value_validator: Option<VF>,
    message: Option<&str>,
) -> ValidateCoreReturnType<HashMap<K, V>>
where
    K: Clone + Eq + Hash,
    V: Clone,
    KF: Fn(&K) -> ValidateCoreReturnType<K>,
    VF: Fn(&V) -> ValidateCoreReturnType<V>,
{
    for (entry_key, entry_value) in value {
        if let Some(ref validate_key) = key_validator {
            let key_result = validate_key(entry_key);
            if !key_result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: key_result.message,
                    type_value: value.clone(),
                };
            }
        }
        if let Some(ref validate_value) = value_validator {
            let value_result = validate_value(entry_value);
            if !value_result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: value_result.message,
                    type_value: value.clone(),
                };
            }
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value.clone(),
    }
}
