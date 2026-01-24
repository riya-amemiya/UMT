use super::umt_math_separator;
use regex::Regex;

/// Expands square of n into a sum of simpler multiplications.
///
/// This function converts expressions like n^2 or n*n into a sum of simpler
/// multiplications using the distributive property. For example, 1250^2 is
/// converted to (1000 + 200 + 50)^2 = 1500*1000 + 400*100 + 200*100 + 50*50
///
/// # Arguments
///
/// * `equation` - Mathematical expression to convert.
///
/// # Returns
///
/// Converted expression.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_math_converter;
///
/// let result = umt_math_converter("1250*1250");
/// assert_eq!(result, "1500*1000+400*100+200*100+50*50");
/// ```
pub fn umt_math_converter(equation: &str) -> String {
    let mut converted_equation = equation.to_string();

    loop {
        let re = Regex::new(r"\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?").unwrap();

        let capture = match re.find(&converted_equation) {
            Some(m) => m.as_str().to_string(),
            None => return converted_equation,
        };

        // Split by * or ^
        let parts: Vec<&str> = capture.split(|c| c == '*' || c == '^').collect();
        if parts.len() != 2 {
            return converted_equation;
        }

        let operand1 = parts[0];
        let operand2 = parts[1];

        // Determine operator
        let operator = if capture.contains('*') { "*" } else { "^" };

        if operand1 == operand2 || operator == "^" {
            let operand1_val: i32 = match operand1.parse() {
                Ok(v) => v,
                Err(_) => return converted_equation,
            };
            let sep_result = umt_math_separator(operand1_val);
            let primary = sep_result[0];
            let remainder = sep_result[1];

            if primary == 0 {
                return converted_equation;
            }

            converted_equation = format!(
                "{}*{}+",
                operand1_val + remainder,
                primary
            );

            if remainder <= 100 {
                converted_equation += &format!("{}*{}", remainder, remainder);
            } else {
                converted_equation += &umt_math_converter(&format!("{}*{}", remainder, remainder));
            }
        } else {
            return converted_equation;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_converter_square() {
        let result = umt_math_converter("1250*1250");
        assert_eq!(result, "1500*1000+400*100+200*100+50*50");
    }

    #[test]
    fn test_math_converter_exponent() {
        let result = umt_math_converter("1250^2");
        assert_eq!(result, "1500*1000+400*100+200*100+50*50");
    }

    #[test]
    fn test_math_converter_small() {
        let result = umt_math_converter("50*50");
        // 50 separates to [10, 40], so 50 + 40 = 90
        assert_eq!(result, "90*10+40*40");
    }

    #[test]
    fn test_math_converter_different_operands() {
        let result = umt_math_converter("10*20");
        assert_eq!(result, "10*20");
    }
}
