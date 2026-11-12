use super::get_luminance::umt_get_luminance;
use super::rgba_to_cmyk::{RgbaError, RgbaInput};

/// Calculate the WCAG 2.1 contrast ratio between two RGBA colors
///
/// # Arguments
///
/// * `color1` - Input containing r, g, b values (0-255) and optional a (0-1)
/// * `color2` - Input containing r, g, b values (0-255) and optional a (0-1)
///
/// # Returns
///
/// Result containing the contrast ratio from 1 (identical luminance) to 21
/// (black on white) or an error. The argument order does not matter.
///
/// # Example
///
/// ```
/// use umt_rust::color::{umt_get_contrast_ratio, RgbaInput};
///
/// let black = RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: None };
/// let white = RgbaInput { r: 255.0, g: 255.0, b: 255.0, a: None };
/// let ratio = umt_get_contrast_ratio(black, white).unwrap();
/// assert!((ratio - 21.0).abs() < 1e-12);
/// ```
pub fn umt_get_contrast_ratio(color1: RgbaInput, color2: RgbaInput) -> Result<f64, RgbaError> {
    let luminance1 = umt_get_luminance(color1)?;
    let luminance2 = umt_get_luminance(color2)?;
    Ok((luminance1.max(luminance2) + 0.05) / (luminance1.min(luminance2) + 0.05))
}
