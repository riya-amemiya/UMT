//! Parity tests for the rgba_to_cmyk module.

use umt_rust::color::{Cmyk, RgbaInput, umt_rgba_to_cmyk};

fn input(r: f64, g: f64, b: f64, a: Option<f64>) -> RgbaInput {
    RgbaInput { r, g, b, a }
}

fn cmyk(c: f64, m: f64, y: f64, k: f64, a: f64) -> Cmyk {
    Cmyk { c, m, y, k, a }
}

#[test]
fn test_rgba_to_cmyk_parity_two_decimal_rounding() {
    assert_eq!(
        umt_rgba_to_cmyk(input(79.0, 160.0, 104.0, Some(0.5))).unwrap(),
        cmyk(50.63, 0.0, 35.0, 37.25, 0.5)
    );
    assert_eq!(
        umt_rgba_to_cmyk(input(193.0, 224.0, 21.0, Some(0.5))).unwrap(),
        cmyk(13.84, 0.0, 90.63, 12.16, 0.5)
    );
    assert_eq!(
        umt_rgba_to_cmyk(input(12.0, 200.0, 255.0, Some(0.5))).unwrap(),
        cmyk(95.29, 21.57, 0.0, 0.0, 0.5)
    );
}

#[test]
fn test_rgba_to_cmyk_parity_black_maps_division_by_zero_to_zero() {
    assert_eq!(
        umt_rgba_to_cmyk(input(0.0, 0.0, 0.0, Some(0.5))).unwrap(),
        cmyk(0.0, 0.0, 0.0, 100.0, 0.5)
    );
}

#[test]
fn test_rgba_to_cmyk_parity_white_and_primary() {
    assert_eq!(
        umt_rgba_to_cmyk(input(255.0, 255.0, 255.0, None)).unwrap(),
        cmyk(0.0, 0.0, 0.0, 0.0, 1.0)
    );
    assert_eq!(
        umt_rgba_to_cmyk(input(255.0, 0.0, 0.0, None)).unwrap(),
        cmyk(0.0, 100.0, 100.0, 0.0, 1.0)
    );
}

#[test]
fn test_rgba_to_cmyk_parity_out_of_range_values_are_errors() {
    assert!(umt_rgba_to_cmyk(input(-1.0, 0.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(256.0, 0.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, -1.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, 256.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, 0.0, -1.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, 0.0, 256.0, None)).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, 0.0, 0.0, Some(-0.1))).is_err());
    assert!(umt_rgba_to_cmyk(input(0.0, 0.0, 0.0, Some(1.1))).is_err());
}
