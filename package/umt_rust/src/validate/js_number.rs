//! JavaScript numeric string coercion helper

/// Determines if a character is JavaScript whitespace or a line terminator,
/// the set trimmed by `Number(string)` before parsing.
fn is_js_whitespace(c: char) -> bool {
    matches!(
        c as u32,
        0x09..=0x0D
            | 0x20
            | 0xA0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
            | 0xFEFF
    )
}

fn parse_radix_literal(literal: &str) -> Option<f64> {
    let bytes = literal.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'0' {
        return None;
    }
    let radix = match bytes[1] {
        b'x' | b'X' => 16,
        b'o' | b'O' => 8,
        b'b' | b'B' => 2,
        _ => return None,
    };
    let digits = &literal[2..];
    if !digits.chars().all(|c| c.is_digit(radix)) {
        return None;
    }
    Some(match u128::from_str_radix(digits, radix) {
        Ok(value) => value as f64,
        Err(_) => digits.chars().fold(0.0, |accumulator, c| {
            accumulator * f64::from(radix) + f64::from(c.to_digit(radix).unwrap_or(0))
        }),
    })
}

fn is_decimal_literal(unsigned: &str) -> bool {
    let bytes = unsigned.as_bytes();
    let integer_digits = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
    let mut index = integer_digits;
    let mut fraction_digits = 0;
    if bytes.get(index) == Some(&b'.') {
        fraction_digits = bytes[index + 1..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        index += 1 + fraction_digits;
    }
    if integer_digits + fraction_digits == 0 {
        return false;
    }
    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let exponent_digits = bytes[index..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        if exponent_digits == 0 {
            return false;
        }
        index += exponent_digits;
    }
    index == bytes.len()
}

/// Converts a string to a number following JavaScript's `Number(string)` rules.
///
/// Surrounding JavaScript whitespace is ignored, the empty string converts to
/// `0`, decimal literals accept an optional sign and exponent, `Infinity` is
/// recognized with an optional sign, and unsigned `0x` / `0o` / `0b` literals
/// are parsed in their radix. Everything else converts to `NaN`.
pub(crate) fn js_string_to_number(value: &str) -> f64 {
    let trimmed = value.trim_matches(is_js_whitespace);
    if trimmed.is_empty() {
        return 0.0;
    }
    if let Some(number) = parse_radix_literal(trimmed) {
        return number;
    }
    let unsigned = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
    if unsigned == "Infinity" {
        return if trimmed.starts_with('-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    if is_decimal_literal(unsigned) {
        return trimmed.parse::<f64>().unwrap_or(f64::NAN);
    }
    f64::NAN
}
