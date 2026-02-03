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
                    if let Some(amount) = amount_string
                        .parse::<f64>()
                        .ok()
                        .filter(|n| n.is_finite())
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
