//! Tests for the hsla_to_rgba module.

use umt_rust::color::{Rgba, umt_hsla_to_rgba};

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
