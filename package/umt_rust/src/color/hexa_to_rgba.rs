use super::cmyk_to_rgba::Rgba;
use regex::Regex;
use std::sync::LazyLock;

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

static HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap());

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
    if !HEX_RE.is_match(hex) {
        return Err(HexColorError {
            message: "Invalid hex code".to_string(),
        });
    }

    // Remove the # prefix
    let digits = &hex.as_bytes()[1..];
    let nibble = |index: usize| f64::from(char::from(digits[index]).to_digit(16).unwrap_or(0));
    let pair = |index: usize| nibble(index) * 16.0 + nibble(index + 1);

    let (r, g, b, alpha_value) = match digits.len() {
        3 => (nibble(0) * 17.0, nibble(1) * 17.0, nibble(2) * 17.0, 255.0),
        8 => (pair(0), pair(2), pair(4), pair(6)),
        _ => (pair(0), pair(2), pair(4), 255.0),
    };

    // Round alpha to 2 decimal places
    let a = (alpha_value / 255.0 * 100.0).round() / 100.0;

    Ok(Rgba { r, g, b, a })
}
