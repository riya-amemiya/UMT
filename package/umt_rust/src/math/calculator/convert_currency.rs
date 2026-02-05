<<<<<<< HEAD
use crate::math::multiplication::umt_multiplication;
use crate::object::Value;
use std::collections::HashMap;

/// Converts currency amounts in a string using currency symbols.
///
/// # Arguments
///
/// * `input_string` - String containing a currency amount to convert
/// * `conversion_rates` - Map of currency symbols to conversion rates
///
/// # Returns
///
/// Returns converted currency amount as a string, or the original string if conversion is not possible.
pub fn umt_convert_currency(
    input_string: &str,
    conversion_rates: Option<&HashMap<String, Value>>,
) -> String {
    if let Some(rates) = conversion_rates {
        for (currency_symbol, rate_val) in rates {
            if input_string.starts_with(currency_symbol) {
                let amount_string = &input_string[currency_symbol.len()..];

                let rate = match rate_val {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
                    _ => f64::NAN,
                };

                #[allow(clippy::collapsible_if)]
                if !rate.is_nan() {
                    if let Some(amount) =
                        amount_string.parse::<f64>().ok().filter(|n| n.is_finite())
                    {
                        let converted_amount = umt_multiplication(&[amount, rate]);
                        if !converted_amount.is_nan() {
                            return converted_amount.to_string();
                        }
                    }
                }
            }
        }
    }
    input_string.to_string()
}
||||||| 55b8153
=======
use std::collections::HashMap;

use crate::math::umt_multiplication;

/// Converts currency amounts in a string using currency symbols.
///
/// # Arguments
///
/// * `input_string` - String containing a currency amount to convert
/// * `conversion_rates` - HashMap mapping currency symbols to conversion rates
///
/// # Returns
///
/// Converted currency amount as a string, or the original string if conversion is not possible.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_convert_currency;
/// use std::collections::HashMap;
///
/// let mut rates = HashMap::new();
/// rates.insert("¥".to_string(), 0.01);
/// assert_eq!(umt_convert_currency("¥100", Some(&rates)), "1");
///
/// let mut rates2 = HashMap::new();
/// rates2.insert("$".to_string(), 1.2);
/// assert_eq!(umt_convert_currency("$50", Some(&rates2)), "60");
/// ```
pub fn umt_convert_currency(
    input_string: &str,
    conversion_rates: Option<&HashMap<String, f64>>,
) -> String {
    let Some(rates) = conversion_rates else {
        return input_string.to_string();
    };

    for (currency_symbol, rate) in rates {
        if input_string.starts_with(currency_symbol) {
            let amount_string = &input_string[currency_symbol.len()..];

            if !rate.is_nan()
                && let Ok(amount) = amount_string.parse::<f64>()
            {
                let converted_amount = umt_multiplication(&[amount, *rate]);
                if !converted_amount.is_nan() {
                    return converted_amount.to_string();
                }
            }
        }
    }

    input_string.to_string()
}

/// Converts currency amounts using string-based rates.
///
/// # Arguments
///
/// * `input_string` - String containing a currency amount to convert
/// * `conversion_rates` - HashMap mapping currency symbols to conversion rates as strings
///
/// # Returns
///
/// Converted currency amount as a string.
pub fn umt_convert_currency_string_rates(
    input_string: &str,
    conversion_rates: Option<&HashMap<String, String>>,
) -> String {
    let Some(rates) = conversion_rates else {
        return input_string.to_string();
    };

    // Convert string rates to f64 rates
    let f64_rates: HashMap<String, f64> = rates
        .iter()
        .filter_map(|(k, v)| v.parse::<f64>().ok().map(|rate| (k.clone(), rate)))
        .collect();

    umt_convert_currency(input_string, Some(&f64_rates))
}
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
