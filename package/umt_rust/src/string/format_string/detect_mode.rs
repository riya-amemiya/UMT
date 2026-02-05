use crate::object::Value;
use std::collections::HashMap;

/// Result of mode detection containing data and options
pub struct DetectModeResult {
    pub data: Value,
    pub options: HashMap<String, Value>,
}

/// Detects whether formatString should use indexed or named mode based on arguments.
///
/// Named mode: First argument is a non-array object
/// Indexed mode: Arguments are treated as array values
///
/// # Arguments
///
/// * `data_or_first_value` - First argument (object for named mode, value for indexed mode)
/// * `options_or_second_value` - Second argument (options for named mode, value for indexed mode)
/// * `rest_values` - Remaining arguments for indexed mode
///
/// # Returns
///
/// Object containing data and options for formatting
pub fn umt_detect_mode(
    data_or_first_value: Option<Value>,
    options_or_second_value: Option<Value>,
    rest_values: Vec<Value>,
) -> DetectModeResult {
    let is_first_argument_object = match &data_or_first_value {
        Some(Value::Object(_)) => true,
        // In Rust Value::Object is definitely not an array, so we just check strict object type
        // TS implementation checks !Array.isArray and !instanceof Date, but Value::Object is map
        _ => false,
    };

    // Check if second argument is options (has formatters property)
    let is_second_argument_options = match &options_or_second_value {
        Some(Value::Object(map)) => map.contains_key("formatters"),
        _ => false,
    };

    // Case 1: Only first argument is object -> Named mode with empty options
    if is_first_argument_object && options_or_second_value.is_none() && rest_values.is_empty() {
        return DetectModeResult {
            data: data_or_first_value.unwrap(),
            options: HashMap::new(),
        };
    }

    // Case 2: First is object, second is options -> Named mode with options
    if is_first_argument_object && is_second_argument_options && rest_values.is_empty() {
        return DetectModeResult {
            data: data_or_first_value.unwrap(),
            options: options_or_second_value.unwrap().into_object().unwrap(),
        };
    }

    // Case 3: Indexed mode (treat all args as data array)
    let mut all_values = Vec::new();
    if let Some(v) = data_or_first_value {
        all_values.push(v);
    }
    if let Some(v) = options_or_second_value {
        all_values.push(v);
    }
    all_values.extend(rest_values);

    DetectModeResult {
        data: Value::Array(all_values),
        options: HashMap::new(),
    }
}
