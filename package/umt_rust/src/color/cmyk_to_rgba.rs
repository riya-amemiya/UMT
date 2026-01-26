/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

/// CMYK color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
    pub a: f64,
}

/// HSLA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

/// Convert CMYK color values to RGBA color space
///
/// # Arguments
///
/// * `c` - Cyan percentage (0-100)
/// * `m` - Magenta percentage (0-100)
/// * `y` - Yellow percentage (0-100)
/// * `k` - Key/Black percentage (0-100)
/// * `a` - Alpha value (0-1), defaults to 1.0
///
/// # Returns
///
/// RGBA values (r, g, b as 0-255, a as 0-1)
///
/// # Example
///
/// ```
/// use umt_rust::color::umt_cmyk_to_rgba;
///
/// let rgba = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 60.78, None);
/// assert_eq!(rgba.r, 0.0);
/// assert_eq!(rgba.g, 0.0);
/// assert_eq!(rgba.b, 100.0);
/// assert_eq!(rgba.a, 1.0);
/// ```
#[inline]
pub fn umt_cmyk_to_rgba(c: f64, m: f64, y: f64, k: f64, a: Option<f64>) -> Rgba {
    let alpha = a.unwrap_or(1.0);

    // Clamp CMYK values to 0-100 range
    let clamped_c = c.clamp(0.0, 100.0);
    let clamped_m = m.clamp(0.0, 100.0);
    let clamped_y = y.clamp(0.0, 100.0);
    let clamped_k = k.clamp(0.0, 100.0);

    // Convert CMYK values to 0-1 range
    let c_percentage = clamped_c / 100.0;
    let m_percentage = clamped_m / 100.0;
    let y_percentage = clamped_y / 100.0;
    let k_percentage = clamped_k / 100.0;

    // Calculate RGB values
    let r = 255.0 * (1.0 - c_percentage) * (1.0 - k_percentage);
    let g = 255.0 * (1.0 - m_percentage) * (1.0 - k_percentage);
    let b = 255.0 * (1.0 - y_percentage) * (1.0 - k_percentage);

    // Round RGB values
    let rounded_r = r.round();
    let rounded_g = g.round();
    let rounded_b = b.round();

    // Clamp alpha value to 0-1 range
    let clamped_a = alpha.clamp(0.0, 1.0);

    Rgba {
        r: rounded_r,
        g: rounded_g,
        b: rounded_b,
        a: clamped_a,
    }
}
