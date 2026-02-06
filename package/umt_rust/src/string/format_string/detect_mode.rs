use super::FormatOptions;
use serde_json::Value;
use std::collections::HashMap;

/// Result of detect_mode function
pub struct DetectModeResult {
    /// The data to be used for formatting
    pub data: Value,
    /// The format options
    pub options: FormatOptions,
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
/// A `DetectModeResult` containing data and options for formatting
///
/// # Examples
///
/// ```
/// use umt_rust::string::format_string::detect_mode::detect_mode;
/// use serde_json::json;
///
/// // Named mode detection
/// let result = detect_mode(Some(&json!({"name": "Alice"})), None, &[]);
/// assert!(result.data.is_object());
///
/// // Indexed mode detection
/// let result = detect_mode(Some(&json!("first")), Some(&json!("second")), &[]);
/// assert!(result.data.is_array());
/// ```
pub fn detect_mode(
    data_or_first_value: Option<&Value>,
    options_or_second_value: Option<&Value>,
    rest_values: &[Value],
) -> DetectModeResult {
    let is_first_argument_object = matches!(data_or_first_value, Some(Value::Object(_)));

    let is_second_argument_options = match options_or_second_value {
        Some(Value::Object(map)) => map.contains_key("formatters"),
        _ => false,
    };

    // Named mode: object only, no extra values
    if is_first_argument_object && options_or_second_value.is_none() && rest_values.is_empty() {
        return DetectModeResult {
            data: data_or_first_value.unwrap().clone(),
            options: FormatOptions::default(),
        };
    }

    // Named mode: object + options, no extra values
    if is_first_argument_object && is_second_argument_options && rest_values.is_empty() {
        return DetectModeResult {
            data: data_or_first_value.unwrap().clone(),
            options: FormatOptions {
                formatters: HashMap::new(),
            },
        };
    }

    // Indexed mode: collect all values into array
    let mut all_values: Vec<Value> = Vec::new();
    if let Some(v) = data_or_first_value {
        all_values.push(v.clone());
    }
    if let Some(v) = options_or_second_value {
        all_values.push(v.clone());
    }
    all_values.extend(rest_values.iter().cloned());

    DetectModeResult {
        data: Value::Array(all_values),
        options: FormatOptions::default(),
    }
}
