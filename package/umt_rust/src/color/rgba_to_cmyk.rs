use super::cmyk_to_rgba::Cmyk;

/// Error type for RGBA color conversion
#[derive(Debug, Clone, PartialEq)]
pub struct RgbaError {
    pub message: String,
}

impl std::fmt::Display for RgbaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RgbaError {}

/// Input struct for RGBA values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaInput {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: Option<f64>,
}

/// Convert RGBA color to CMYK color model
///
/// # Arguments
///
/// * `rgba` - Input containing r, g, b values (0-255) and optional a (0-1)
///
/// # Returns
///
/// Result containing CMYK values (c, m, y, k as percentages 0-100) and alpha channel or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::{umt_rgba_to_cmyk, RgbaInput};
///
/// let cmyk = umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap();
/// assert_eq!(cmyk.c, 0.0);
/// assert_eq!(cmyk.m, 0.0);
/// assert_eq!(cmyk.y, 0.0);
/// assert_eq!(cmyk.k, 100.0);
/// assert_eq!(cmyk.a, 1.0);
/// ```
#[inline]
pub fn umt_rgba_to_cmyk(rgba: RgbaInput) -> Result<Cmyk, RgbaError> {
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

    let max_val = r_prime.max(g_prime).max(b_prime);
    let k = 1.0 - max_val;

    let (c, m, y) = if (1.0 - k).abs() < f64::EPSILON {
        // Black - avoid division by zero
        (0.0, 0.0, 0.0)
    } else {
        let c = (1.0 - r_prime - k) / (1.0 - k);
        let m = (1.0 - g_prime - k) / (1.0 - k);
        let y = (1.0 - b_prime - k) / (1.0 - k);
        (c, m, y)
    };

    // Round to 2 decimal places
    let round_to_2 = |x: f64| (x * 100.0).round() / 100.0;

    Ok(Cmyk {
        c: round_to_2(c * 100.0),
        m: round_to_2(m * 100.0),
        y: round_to_2(y * 100.0),
        k: round_to_2(k * 100.0),
        a: alpha,
    })
}
