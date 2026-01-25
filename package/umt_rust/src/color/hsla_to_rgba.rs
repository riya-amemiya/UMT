use super::cmyk_to_rgba::Rgba;

/// Error type for HSLA color conversion
#[derive(Debug, Clone, PartialEq)]
pub struct HslaError {
    pub message: String,
}

impl std::fmt::Display for HslaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HslaError {}

/// Convert HSLA color values to RGBA color space
///
/// # Arguments
///
/// * `h` - Hue angle in degrees (0-360)
/// * `s` - Saturation percentage (0-100)
/// * `l` - Lightness percentage (0-100)
/// * `a` - Alpha value (0-1), defaults to 1.0
///
/// # Returns
///
/// Result containing RGBA values (r, g, b as 0-255, a as 0-1) or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::umt_hsla_to_rgba;
///
/// let rgba = umt_hsla_to_rgba(0.0, 100.0, 50.0, None).unwrap();
/// assert_eq!(rgba.r, 255.0);
/// assert_eq!(rgba.g, 0.0);
/// assert_eq!(rgba.b, 0.0);
/// assert_eq!(rgba.a, 1.0);
/// ```
#[inline]
pub fn umt_hsla_to_rgba(h: f64, s: f64, l: f64, a: Option<f64>) -> Result<Rgba, HslaError> {
    let alpha = a.unwrap_or(1.0);

    // Validate input ranges
    if !(0.0..=360.0).contains(&h) {
        return Err(HslaError {
            message: "Hue must be between 0 and 360 degrees".to_string(),
        });
    }
    if !(0.0..=100.0).contains(&s) {
        return Err(HslaError {
            message: "Saturation must be between 0 and 100 percent".to_string(),
        });
    }
    if !(0.0..=100.0).contains(&l) {
        return Err(HslaError {
            message: "Lightness must be between 0 and 100 percent".to_string(),
        });
    }
    if !(0.0..=1.0).contains(&alpha) {
        return Err(HslaError {
            message: "Alpha must be between 0 and 1".to_string(),
        });
    }

    // Normalize values
    let hue = (h % 360.0) / 360.0;
    let saturation = s.clamp(0.0, 100.0) / 100.0;
    let lightness = l.clamp(0.0, 100.0) / 100.0;

    let (r, g, b) = if saturation == 0.0 {
        (lightness, lightness, lightness)
    } else {
        let hue_to_rgb = |p: f64, q: f64, t: f64| -> f64 {
            let mut t_adjusted = t;
            if t < 0.0 {
                t_adjusted = t + 1.0;
            }
            if t > 1.0 {
                t_adjusted = t - 1.0;
            }
            if t_adjusted < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t_adjusted;
            }
            if t_adjusted < 1.0 / 2.0 {
                return q;
            }
            if t_adjusted < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t_adjusted) * 6.0;
            }
            p
        };

        let q = if lightness < 0.5 {
            lightness * (1.0 + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };
        let p = 2.0 * lightness - q;

        let r = hue_to_rgb(p, q, hue + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, hue);
        let b = hue_to_rgb(p, q, hue - 1.0 / 3.0);

        (r, g, b)
    };

    // Round to 2 decimal places
    let round_to_2 = |x: f64| (x * 100.0).round() / 100.0;
    let rounded_r = round_to_2(r * 255.0);
    let rounded_g = round_to_2(g * 255.0);
    let rounded_b = round_to_2(b * 255.0);
    let clamped_a = alpha.clamp(0.0, 1.0);

    Ok(Rgba {
        r: rounded_r,
        g: rounded_g,
        b: rounded_b,
        a: clamped_a,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsla_to_rgba_primary_colors() {
        // Red
        assert_eq!(
            umt_hsla_to_rgba(0.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        // Green
        assert_eq!(
            umt_hsla_to_rgba(120.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: 1.0
            }
        );
        // Blue
        assert_eq!(
            umt_hsla_to_rgba(240.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_secondary_colors() {
        // Yellow
        assert_eq!(
            umt_hsla_to_rgba(60.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 255.0,
                g: 255.0,
                b: 0.0,
                a: 1.0
            }
        );
        // Cyan
        assert_eq!(
            umt_hsla_to_rgba(180.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 255.0,
                b: 255.0,
                a: 1.0
            }
        );
        // Magenta
        assert_eq!(
            umt_hsla_to_rgba(300.0, 100.0, 50.0, Some(1.0)).unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 255.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_black_white() {
        // White
        assert_eq!(
            umt_hsla_to_rgba(0.0, 0.0, 100.0, Some(1.0)).unwrap(),
            Rgba {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 1.0
            }
        );
        // Black
        assert_eq!(
            umt_hsla_to_rgba(0.0, 0.0, 0.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_with_alpha() {
        assert_eq!(
            umt_hsla_to_rgba(0.0, 100.0, 50.0, Some(0.5)).unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 0.5
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_default_alpha() {
        assert_eq!(
            umt_hsla_to_rgba(0.0, 0.0, 0.0, None).unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hsla_to_rgba(360.0, 100.0, 100.0, None).unwrap(),
            Rgba {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 1.0
            }
        );
        assert_eq!(
            umt_hsla_to_rgba(0.0, 100.0, 50.0, None).unwrap(),
            Rgba {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_dark_colors() {
        // Dark blue
        assert_eq!(
            umt_hsla_to_rgba(240.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 127.5,
                a: 1.0
            }
        );
        // Dark green
        assert_eq!(
            umt_hsla_to_rgba(120.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 127.5,
                b: 0.0,
                a: 1.0
            }
        );
        // Dark red
        assert_eq!(
            umt_hsla_to_rgba(0.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 127.5,
                g: 0.0,
                b: 0.0,
                a: 1.0
            }
        );
        // Dark yellow
        assert_eq!(
            umt_hsla_to_rgba(60.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 127.5,
                g: 127.5,
                b: 0.0,
                a: 1.0
            }
        );
        // Dark cyan
        assert_eq!(
            umt_hsla_to_rgba(180.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 0.0,
                g: 127.5,
                b: 127.5,
                a: 1.0
            }
        );
        // Dark magenta
        assert_eq!(
            umt_hsla_to_rgba(300.0, 100.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 127.5,
                g: 0.0,
                b: 127.5,
                a: 1.0
            }
        );
        // Gray (0% saturation)
        assert_eq!(
            umt_hsla_to_rgba(0.0, 0.0, 25.0, Some(1.0)).unwrap(),
            Rgba {
                r: 63.75,
                g: 63.75,
                b: 63.75,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_hsla_to_rgba_invalid_hue() {
        let err = umt_hsla_to_rgba(-60.0, 100.0, 50.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Hue must be between 0 and 360 degrees");

        let err = umt_hsla_to_rgba(420.0, 100.0, 50.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Hue must be between 0 and 360 degrees");
    }

    #[test]
    fn test_hsla_to_rgba_invalid_saturation() {
        let err = umt_hsla_to_rgba(0.0, -50.0, 50.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Saturation must be between 0 and 100 percent");

        let err = umt_hsla_to_rgba(0.0, 150.0, 50.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Saturation must be between 0 and 100 percent");
    }

    #[test]
    fn test_hsla_to_rgba_invalid_lightness() {
        let err = umt_hsla_to_rgba(0.0, 100.0, -20.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Lightness must be between 0 and 100 percent");

        let err = umt_hsla_to_rgba(0.0, 100.0, 120.0, Some(1.0)).unwrap_err();
        assert_eq!(err.message, "Lightness must be between 0 and 100 percent");
    }

    #[test]
    fn test_hsla_to_rgba_invalid_alpha() {
        let err = umt_hsla_to_rgba(0.0, 100.0, 50.0, Some(-0.5)).unwrap_err();
        assert_eq!(err.message, "Alpha must be between 0 and 1");

        let err = umt_hsla_to_rgba(0.0, 100.0, 50.0, Some(1.5)).unwrap_err();
        assert_eq!(err.message, "Alpha must be between 0 and 1");
    }
}
