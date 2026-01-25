use super::rgba_to_cmyk::{RgbaError, RgbaInput};

/// Convert RGBA color to hexadecimal color code
///
/// # Arguments
///
/// * `rgba` - Input containing r, g, b values (0-255) and optional a (0-1)
///
/// # Returns
///
/// Result containing hexadecimal color code including alpha channel or an error
///
/// # Example
///
/// ```
/// use umt_rust::color::{umt_rgba_to_hexa, RgbaInput};
///
/// let hex = umt_rgba_to_hexa(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap();
/// assert_eq!(hex, "#ff0000ff");
/// ```
#[inline]
pub fn umt_rgba_to_hexa(rgba: RgbaInput) -> Result<String, RgbaError> {
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

    // Helper function to convert a number to 2-digit hex
    let to_hex = |x: u8| -> String { format!("{:02x}", x) };

    let r_hex = to_hex(r as u8);
    let g_hex = to_hex(g as u8);
    let b_hex = to_hex(b as u8);
    let a_hex = to_hex((alpha * 255.0).round() as u8);

    Ok(format!("#{}{}{}{}", r_hex, g_hex, b_hex, a_hex))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_to_hexa_valid() {
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: None
            })
            .unwrap(),
            "#ff0000ff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: Some(1.0)
            })
            .unwrap(),
            "#00ff00ff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: Some(1.0)
            })
            .unwrap(),
            "#0000ffff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: Some(1.0)
            })
            .unwrap(),
            "#ffffffff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0)
            })
            .unwrap(),
            "#000000ff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 255.0,
                g: 165.0,
                b: 0.0,
                a: Some(1.0)
            })
            .unwrap(),
            "#ffa500ff"
        );
        assert_eq!(
            umt_rgba_to_hexa(RgbaInput {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: Some(0.5)
            })
            .unwrap(),
            "#ffffff80"
        );
    }

    #[test]
    fn test_rgba_to_hexa_invalid() {
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 256.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 256.0,
                b: 0.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 256.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.5)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: -1.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: -1.0,
                b: 0.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: -1.0,
                a: Some(1.0)
            })
            .is_err()
        );
        assert!(
            umt_rgba_to_hexa(RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: Some(-0.5)
            })
            .is_err()
        );
    }

    #[test]
    fn test_rgba_to_hexa_error_message() {
        let err = umt_rgba_to_hexa(RgbaInput {
            r: 256.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.0),
        })
        .unwrap_err();
        assert_eq!(err.message, "Invalid rgba value");
    }
}
