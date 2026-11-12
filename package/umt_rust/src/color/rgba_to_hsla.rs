use super::cmyk_to_rgba::Hsla;
use super::rgba_to_cmyk::{RgbaError, RgbaInput};
use crate::math::{umt_addition, umt_division, umt_multiplication, umt_round_of, umt_subtract};

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

    let r_prime = umt_division(r, 255.0);
    let g_prime = umt_division(g, 255.0);
    let b_prime = umt_division(b, 255.0);

    let max = r_prime.max(g_prime).max(b_prime);
    let min = r_prime.min(g_prime).min(b_prime);
    let diff = umt_subtract(&[max, min]);
    let sum = umt_addition(&[max, min]);

    let l = umt_division(sum, 2.0);

    let s = if diff == 0.0 {
        0.0
    } else {
        umt_division(diff, umt_subtract(&[1.0, umt_subtract(&[sum, 1.0]).abs()]))
    };

    let h = if diff == 0.0 {
        0.0
    } else if max == r_prime {
        let offset = if g_prime < b_prime { 6.0 } else { 0.0 };
        umt_multiplication(&[
            umt_addition(&[
                umt_division(umt_subtract(&[g_prime, b_prime]), diff),
                offset,
            ]),
            60.0,
        ])
    } else if max == g_prime {
        umt_multiplication(&[
            umt_addition(&[umt_division(umt_subtract(&[b_prime, r_prime]), diff), 2.0]),
            60.0,
        ])
    } else {
        umt_multiplication(&[
            umt_addition(&[umt_division(umt_subtract(&[r_prime, g_prime]), diff), 4.0]),
            60.0,
        ])
    };

    Ok(Hsla {
        h: umt_round_of(h, 2),
        s: umt_round_of(s * 100.0, 2),
        l: umt_round_of(l * 100.0, 2),
        a: alpha,
    })
}
