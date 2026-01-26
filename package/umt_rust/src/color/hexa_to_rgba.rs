use super::cmyk_to_rgba::Rgba;

/// Error type for hex color parsing
#[derive(Debug, Clone, PartialEq)]
pub struct HexColorError {
    pub message: String,
}

impl std::fmt::Display for HexColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HexColorError {}

fn is_valid_hex_color(hex: &str) -> bool {
    if !hex.starts_with('#') {
        return false;
    }
    let hex_part = &hex[1..];
    let len = hex_part.len();
    if len != 3 && len != 6 && len != 8 {
        return false;
    }
    hex_part.chars().all(|c| c.is_ascii_hexdigit())
}

/// Convert hexadecimal color code to RGBA color values
///
/// # Arguments
///
/// * `hex` - Hexadecimal color code (3, 6, or 8 digits with #)
///
/// # Returns
///
/// Result containing RGBA values (r, g, b as 0-255, a as 0-1) or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::umt_hexa_to_rgba;
///
/// let rgba = umt_hexa_to_rgba("#FF0000").unwrap();
/// assert_eq!(rgba.r, 255.0);
/// assert_eq!(rgba.g, 0.0);
/// assert_eq!(rgba.b, 0.0);
/// assert_eq!(rgba.a, 1.0);
/// ```
#[inline]
pub fn umt_hexa_to_rgba(hex: &str) -> Result<Rgba, HexColorError> {
    if !is_valid_hex_color(hex) {
        return Err(HexColorError {
            message: "Invalid hex code".to_string(),
        });
    }

    // Remove the # prefix
    let hex_code = &hex[1..];

    // Convert 3-digit hex to 6-digit format
    let expanded_hex = if hex_code.len() == 3 {
        hex_code
            .chars()
            .map(|c| format!("{}{}", c, c))
            .collect::<String>()
    } else {
        hex_code.to_string()
    };

    // Parse RGB values
    let r = u8::from_str_radix(&expanded_hex[0..2], 16).unwrap() as f64;
    let g = u8::from_str_radix(&expanded_hex[2..4], 16).unwrap() as f64;
    let b = u8::from_str_radix(&expanded_hex[4..6], 16).unwrap() as f64;

    // Parse alpha value (default to 255 if not provided)
    let alpha_value = if expanded_hex.len() == 8 {
        u8::from_str_radix(&expanded_hex[6..8], 16).unwrap() as f64
    } else {
        255.0
    };

    // Round alpha to 2 decimal places
    let a = (alpha_value / 255.0 * 100.0).round() / 100.0;

    Ok(Rgba { r, g, b, a })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexa_to_rgba_6_digit() {
        assert_eq!(
            umt_hexa_to_rgba("#FF0000").unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#00FF00").unwrap(),
            Rgba {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#0000FF").unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FFFFFF").unwrap(),
            Rgba {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#000000").unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FFA500").unwrap(),
            Rgba {
                r: 255.0,
                g: 165.0,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hexa_to_rgba_8_digit() {
        assert_eq!(
            umt_hexa_to_rgba("#FFA50099").unwrap(),
            Rgba {
                r: 255.0,
                g: 165.0,
                b: 0.0,
                a: 0.6
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FF0000FF").unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FF000080").unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 0.5
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FF000000").unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 0.0
            }
        );
    }

    #[test]
    fn test_hexa_to_rgba_3_digit() {
        assert_eq!(
            umt_hexa_to_rgba("#F00").unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#0F0").unwrap(),
            Rgba {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#00F").unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#FFF").unwrap(),
            Rgba {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hexa_to_rgba("#000").unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hexa_to_rgba_invalid() {
        assert!(umt_hexa_to_rgba("#12345").is_err());
        assert!(umt_hexa_to_rgba("#1234567").is_err());
        assert!(umt_hexa_to_rgba("123456").is_err());
        assert!(umt_hexa_to_rgba("").is_err());
    }

    #[test]
    fn test_hexa_to_rgba_error_message() {
        let err = umt_hexa_to_rgba("#12345").unwrap_err();
        assert_eq!(err.message, "Invalid hex code");
    }
}
