use super::cmyk_to_rgba::Rgba;
use regex::Regex;

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
    // Validate hex code format using regex
    let re = Regex::new(r"^#([\da-fA-F]{3}|[\da-fA-F]{6}|[\da-fA-F]{8})$").unwrap();

    if !re.is_match(hex) {
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
