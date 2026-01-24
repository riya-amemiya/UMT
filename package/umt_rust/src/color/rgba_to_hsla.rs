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
    if r < 0.0 || r > 255.0 || g < 0.0 || g > 255.0 || b < 0.0 || b > 255.0 || alpha < 0.0 || alpha > 1.0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_to_hsla_invalid_values() {
        assert!(umt_rgba_to_hsla(RgbaInput { r: -1.0, g: 0.0, b: 0.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 256.0, g: 0.0, b: 0.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: -1.0, b: 0.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 256.0, b: 0.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: -1.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 256.0, a: Some(1.0) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(-0.1) }).is_err());
        assert!(umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(1.1) }).is_err());
    }

    #[test]
    fn test_rgba_to_hsla_error_message() {
        let err = umt_rgba_to_hsla(RgbaInput { r: -1.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap_err();
        assert_eq!(err.message, "Invalid rgba value");
    }

    #[test]
    fn test_rgba_to_hsla_black() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 0.0, s: 0.0, l: 0.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_white() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 255.0, b: 255.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 0.0, s: 0.0, l: 100.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_gray() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 100.0, g: 100.0, b: 100.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 0.0, s: 0.0, l: 39.22, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_red() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 0.0, s: 100.0, l: 50.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_green() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 255.0, b: 0.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 120.0, s: 100.0, l: 50.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_blue() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 255.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 240.0, s: 100.0, l: 50.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_with_alpha() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: Some(0.5) }).unwrap();
        assert_eq!(result, Hsla { h: 0.0, s: 100.0, l: 50.0, a: 0.5 });
    }

    #[test]
    fn test_rgba_to_hsla_yellow() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 255.0, b: 0.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 60.0, s: 100.0, l: 50.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_blue_with_alpha() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 0.0, g: 0.0, b: 255.0, a: Some(0.7) }).unwrap();
        assert_eq!(result, Hsla { h: 240.0, s: 100.0, l: 50.0, a: 0.7 });
    }

    #[test]
    fn test_rgba_to_hsla_light_blue() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 173.0, g: 216.0, b: 230.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 194.74, s: 53.27, l: 79.02, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_pink() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 255.0, g: 0.0, b: 100.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 336.47, s: 100.0, l: 50.0, a: 1.0 });
    }

    #[test]
    fn test_rgba_to_hsla_purple() {
        let result = umt_rgba_to_hsla(RgbaInput { r: 100.0, g: 0.0, b: 255.0, a: None }).unwrap();
        assert_eq!(result, Hsla { h: 263.53, s: 100.0, l: 50.0, a: 1.0 });
    }
}
