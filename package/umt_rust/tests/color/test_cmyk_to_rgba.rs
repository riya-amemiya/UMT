//! Tests for the cmyk_to_rgba module.

use umt_rust::color::{Rgba, umt_cmyk_to_rgba};

#[test]
fn test_cmyk_to_rgba_basic_conversions() {
    // Dark blue
    let result = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 60.78, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 100.0,
            a: 1.0
        }
    );

    // White
    let result = umt_cmyk_to_rgba(0.0, 0.0, 0.0, 0.0, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0
        }
    );

    // Red with alpha 0.5
    let result = umt_cmyk_to_rgba(0.0, 100.0, 100.0, 0.0, Some(0.5));
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 0.5
        }
    );

    // Green with alpha 0.7
    let result = umt_cmyk_to_rgba(100.0, 0.0, 100.0, 0.0, Some(0.7));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 255.0,
            b: 0.0,
            a: 0.7
        }
    );

    // Blue with alpha 0.3
    let result = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 0.0, Some(0.3));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 255.0,
            a: 0.3
        }
    );

    // Gray
    let result = umt_cmyk_to_rgba(50.0, 50.0, 50.0, 50.0, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 64.0,
            g: 64.0,
            b: 64.0,
            a: 1.0
        }
    );
}

#[test]
fn test_cmyk_to_rgba_handles_invalid_values() {
    // Negative C value should be clamped to 0
    let result = umt_cmyk_to_rgba(-1.0, 100.0, 100.0, 0.0, None);
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );

    // All max values (black)
    let result = umt_cmyk_to_rgba(100.0, 100.0, 100.0, 100.0, None);
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );

    // Negative alpha should be clamped to 0
    let result = umt_cmyk_to_rgba(100.0, 100.0, 100.0, 100.0, Some(-1.0));
    assert_eq!(result.a, 0.0);
}

#[test]
fn test_cmyk_to_rgba_default_alpha() {
    let result = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 60.78, None);
    assert_eq!(result.a, 1.0);
}

use umt_rust::color::*;

#[test]
fn test_cmyk_to_rgba_basic() {
    let result = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 60.78, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 100.0,
            a: 1.0
        }
    );

    let result = umt_cmyk_to_rgba(0.0, 0.0, 0.0, 0.0, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0
        }
    );

    let result = umt_cmyk_to_rgba(0.0, 100.0, 100.0, 0.0, Some(0.5));
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 0.5
        }
    );

    let result = umt_cmyk_to_rgba(100.0, 0.0, 100.0, 0.0, Some(0.7));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 255.0,
            b: 0.0,
            a: 0.7
        }
    );

    let result = umt_cmyk_to_rgba(100.0, 100.0, 0.0, 0.0, Some(0.3));
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 255.0,
            a: 0.3
        }
    );

    let result = umt_cmyk_to_rgba(50.0, 50.0, 50.0, 50.0, Some(1.0));
    assert_eq!(
        result,
        Rgba {
            r: 64.0,
            g: 64.0,
            b: 64.0,
            a: 1.0
        }
    );
}

#[test]
fn test_cmyk_to_rgba_clamp_values() {
    // Negative values should be clamped to 0
    let result = umt_cmyk_to_rgba(-1.0, 100.0, 100.0, 0.0, None);
    assert_eq!(
        result,
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );

    // Maximum k value
    let result = umt_cmyk_to_rgba(100.0, 100.0, 100.0, 100.0, None);
    assert_eq!(
        result,
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );

    // Negative alpha should be clamped
    let result = umt_cmyk_to_rgba(100.0, 100.0, 100.0, 100.0, Some(-1.0));
    assert_eq!(result.a, 0.0);
}
