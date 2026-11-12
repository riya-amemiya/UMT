use super::rgba_to_cmyk::{RgbaError, RgbaInput};

/// Converts one sRGB channel (0-255) to its linear value (0-1).
fn to_linear(channel: f64) -> f64 {
    let srgb = channel / 255.0;
    if srgb <= 0.03928 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate the WCAG 2.1 relative luminance of an RGBA color
///
/// Alpha is validated but does not affect the result.
///
/// # Arguments
///
/// * `rgba` - Input containing r, g, b values (0-255) and optional a (0-1)
///
/// # Returns
///
/// Result containing the relative luminance from 0 (black) to 1 (white) or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::{umt_get_luminance, RgbaInput};
///
/// let white = umt_get_luminance(RgbaInput { r: 255.0, g: 255.0, b: 255.0, a: None }).unwrap();
/// assert!((white - 1.0).abs() < 1e-12);
/// let red = umt_get_luminance(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap();
/// assert!((red - 0.2126).abs() < 1e-12);
/// ```
pub fn umt_get_luminance(rgba: RgbaInput) -> Result<f64, RgbaError> {
    let RgbaInput { r, g, b, a } = rgba;

    if !(0.0..=255.0).contains(&r)
        || !(0.0..=255.0).contains(&g)
        || !(0.0..=255.0).contains(&b)
        || !(0.0..=1.0).contains(&a.unwrap_or(1.0))
    {
        return Err(RgbaError {
            message: "Invalid rgba value".to_string(),
        });
    }

    Ok(0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b))
}
