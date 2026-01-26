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
