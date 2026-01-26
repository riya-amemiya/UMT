use super::cmyk_to_rgba::Hsla;
use super::rgba_to_cmyk::{RgbaError, RgbaInput};

/// Convert RGBA color values to HSLA color space
///
/// # Arguments
///
/// * `rgba` - Input containing r, g, b values (0-255) and optional a (0-1)
///
/// # Returns
///
/// Result containing HSLA values (h as 0-360, s and l as 0-100, a as 0-1) or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::{umt_rgba_to_hsla, RgbaInput};
///
/// let hsla = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap();
/// assert_eq!(hsla.h, 0.0);
/// assert_eq!(hsla.s, 100.0);
/// assert_eq!(hsla.l, 50.0);
/// assert_eq!(hsla.a, 1.0);
/// ```
#[inline]
pub fn umt_rgba_to_hsla(rgba: RgbaInput) -> Result<Hsla, RgbaError> {
    let RgbaInput { r, g, b, a } = rgba;
    let alpha = a.unwrap_or(1.0);

    // Validate RGBA values
    if !(0.0..=255.0).contains(&r)
        || !(0.0..=255.0).contains(&g)
        || !(0.0..=255.0).contains(&b)
        || !(0.0..=1.0).contains(&alpha)
    {
        return Err(RgbaError {
            message: "Invalid rgba value".to_string(),
        });
    }

    let r_prime = r / 255.0;
    let g_prime = g / 255.0;
    let b_prime = b / 255.0;

    let max = r_prime.max(g_prime).max(b_prime);
    let min = r_prime.min(g_prime).min(b_prime);
    let diff = max - min;

    let l = (max + min) / 2.0;

    let s = if diff == 0.0 {
        0.0
    } else {
        diff / (1.0 - (max + min - 1.0).abs())
    };

    let mut h = 0.0;

    if diff != 0.0 {
        if (max - r_prime).abs() < f64::EPSILON {
            h = (g_prime - b_prime) / diff + if g_prime < b_prime { 6.0 } else { 0.0 };
        } else if (max - g_prime).abs() < f64::EPSILON {
            h = (b_prime - r_prime) / diff + 2.0;
        } else if (max - b_prime).abs() < f64::EPSILON {
            h = (r_prime - g_prime) / diff + 4.0;
        }
        h *= 60.0;
    }

    // Round to 2 decimal places
    let round_to_2 = |x: f64| (x * 100.0).round() / 100.0;

    Ok(Hsla {
        h: round_to_2(h),
        s: round_to_2(s * 100.0),
        l: round_to_2(l * 100.0),
        a: alpha,
    })
}
