use regex::Regex;
use std::collections::HashMap;
use crate::object::Value;

#[allow(clippy::collapsible_if)]
pub fn umt_convert_currency(
    expression: &str,
    currency_exchange: Option<&HashMap<String, Value>>,
) -> String {
    let re = Regex::new(r"(\D+)([\d.]+)").unwrap();

    if let Some(caps) = re.captures(expression) {
        let currency = caps.get(1).map_or("", |m| m.as_str());
        let amount_str = caps.get(2).map_or("0", |m| m.as_str());

        if let Ok(amount) = amount_str.parse::<f64>() {
            if let Some(exchange_rates) = currency_exchange {
                if let Some(rate_val) = exchange_rates.get(currency) {
                    let rate = match rate_val {
                        Value::Int(i) => *i as f64,
                        Value::Float(f) => *f,
                        _ => 1.0,
                    };

                    if rate != 0.0 {
                        // Check if rate is effectively 1.0
                        if (rate - 1.0).abs() < f64::EPSILON {
                            return amount.to_string();
                        }
                        let result = amount * rate;
                        // Handle floating point precision issues
                        let rounded = (result * 1e10).round() / 1e10;
                        return rounded.to_string();
                    }
                }
            }
        }
    }

    expression.to_string()
}
