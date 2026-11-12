use super::cmyk_to_rgba::Rgba;
use crate::math::{
    umt_addition, umt_clamp, umt_division, umt_division_with_remainder, umt_multiplication,
    umt_round_of, umt_subtract,
};

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
    let hue_remainder = umt_division_with_remainder(h, 360.0).map_or(0, |result| result.remainder);
    let hue = umt_division(hue_remainder as f64, 360.0);
    let saturation = umt_division(umt_clamp(s, 0.0, 100.0), 100.0);
    let lightness = umt_division(umt_clamp(l, 0.0, 100.0), 100.0);

    let (r, g, b) = if saturation == 0.0 {
        (lightness, lightness, lightness)
    } else {
        let one_third = umt_division(1.0, 3.0);
        let two_thirds = umt_division(2.0, 3.0);
        let hue_to_rgb = |p: f64, q: f64, t: f64| -> f64 {
            let t_adjusted = if t > 1.0 {
                umt_subtract(&[t, 1.0])
            } else if t < 0.0 {
                umt_addition(&[t, 1.0])
            } else {
                t
            };
            if t_adjusted < umt_division(1.0, 6.0) {
                return umt_addition(&[
                    p,
                    umt_multiplication(&[
                        umt_subtract(&[q, p]),
                        umt_multiplication(&[6.0, t_adjusted]),
                    ]),
                ]);
            }
            if t_adjusted < umt_division(1.0, 2.0) {
                return q;
            }
            if t_adjusted < two_thirds {
                return umt_addition(&[
                    p,
                    umt_multiplication(&[
                        umt_subtract(&[q, p]),
                        umt_multiplication(&[umt_subtract(&[two_thirds, t_adjusted]), 6.0]),
                    ]),
                ]);
            }
            p
        };

        let q = if lightness < 0.5 {
            umt_multiplication(&[lightness, umt_addition(&[1.0, saturation])])
        } else {
            umt_subtract(&[
                umt_addition(&[lightness, saturation]),
                umt_multiplication(&[lightness, saturation]),
            ])
        };
        let p = umt_subtract(&[umt_multiplication(&[2.0, lightness]), q]);

        let r = hue_to_rgb(p, q, umt_addition(&[hue, one_third]));
        let g = hue_to_rgb(p, q, hue);
        let b = hue_to_rgb(p, q, umt_subtract(&[hue, one_third]));

        (r, g, b)
    };

    Ok(Rgba {
        r: umt_round_of(umt_multiplication(&[r, 255.0]), 2),
        g: umt_round_of(umt_multiplication(&[g, 255.0]), 2),
        b: umt_round_of(umt_multiplication(&[b, 255.0]), 2),
        a: umt_clamp(alpha, 0.0, 1.0),
    })
}
