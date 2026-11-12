//! Parity tests for the hsla_to_rgba module.

use umt_rust::color::{Rgba, umt_hsla_to_rgba};

fn rgba(r: f64, g: f64, b: f64, a: f64) -> Rgba {
    Rgba { r, g, b, a }
}

#[test]
fn test_hsla_to_rgba_parity_two_decimal_rounding() {
    assert_eq!(
        umt_hsla_to_rgba(0.0, 1.0, 70.0, Some(0.1)).unwrap(),
        rgba(179.27, 177.74, 177.74, 0.1)
    );
    assert_eq!(
        umt_hsla_to_rgba(0.0, 50.0, 75.0, Some(0.25)).unwrap(),
        rgba(223.13, 159.38, 159.38, 0.25)
    );
}

#[test]
fn test_hsla_to_rgba_parity_lightness_below_half() {
    assert_eq!(
        umt_hsla_to_rgba(120.0, 50.0, 50.0, Some(1.0)).unwrap(),
        rgba(63.75, 191.25, 63.75, 1.0)
    );
    assert_eq!(
        umt_hsla_to_rgba(90.0, 25.0, 20.0, None).unwrap(),
        rgba(51.0, 63.75, 38.25, 1.0)
    );
}

#[test]
fn test_hsla_to_rgba_parity_lightness_at_or_above_half() {
    assert_eq!(
        umt_hsla_to_rgba(210.0, 40.0, 70.0, Some(0.3)).unwrap(),
        rgba(147.9, 178.5, 209.1, 0.3)
    );
}

#[test]
fn test_hsla_to_rgba_parity_every_hue_segment() {
    assert_eq!(
        umt_hsla_to_rgba(30.0, 100.0, 50.0, None).unwrap(),
        rgba(255.0, 127.5, 0.0, 1.0)
    );
    assert_eq!(
        umt_hsla_to_rgba(200.0, 100.0, 50.0, None).unwrap(),
        rgba(0.0, 170.0, 255.0, 1.0)
    );
    assert_eq!(
        umt_hsla_to_rgba(270.0, 60.0, 40.0, None).unwrap(),
        rgba(102.0, 40.8, 163.2, 1.0)
    );
}

#[test]
fn test_hsla_to_rgba_parity_hue_360_wraps_to_zero() {
    assert_eq!(
        umt_hsla_to_rgba(360.0, 100.0, 50.0, None).unwrap(),
        rgba(255.0, 0.0, 0.0, 1.0)
    );
}

#[test]
fn test_hsla_to_rgba_parity_zero_saturation_is_gray() {
    assert_eq!(
        umt_hsla_to_rgba(0.0, 0.0, 50.0, None).unwrap(),
        rgba(127.5, 127.5, 127.5, 1.0)
    );
}

#[test]
fn test_hsla_to_rgba_parity_fractional_hue_uses_typescript_digit_remainder() {
    assert_eq!(
        umt_hsla_to_rgba(120.5, 50.0, 50.0, None).unwrap(),
        rgba(63.75, 191.25, 74.38, 1.0)
    );
}

#[test]
fn test_hsla_to_rgba_parity_out_of_range_values_are_errors() {
    assert!(umt_hsla_to_rgba(-1.0, 50.0, 50.0, None).is_err());
    assert!(umt_hsla_to_rgba(361.0, 50.0, 50.0, None).is_err());
    assert!(umt_hsla_to_rgba(0.0, -1.0, 50.0, None).is_err());
    assert!(umt_hsla_to_rgba(0.0, 101.0, 50.0, None).is_err());
    assert!(umt_hsla_to_rgba(0.0, 50.0, -1.0, None).is_err());
    assert!(umt_hsla_to_rgba(0.0, 50.0, 101.0, None).is_err());
    assert!(umt_hsla_to_rgba(0.0, 50.0, 50.0, Some(-0.1)).is_err());
    assert!(umt_hsla_to_rgba(0.0, 50.0, 50.0, Some(1.1)).is_err());
}
