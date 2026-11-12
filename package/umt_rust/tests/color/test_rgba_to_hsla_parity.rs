//! Parity tests for the rgba_to_hsla module.

use umt_rust::color::{Hsla, RgbaInput, umt_rgba_to_hsla};

fn input(r: f64, g: f64, b: f64, a: Option<f64>) -> RgbaInput {
    RgbaInput { r, g, b, a }
}

fn hsla(h: f64, s: f64, l: f64, a: f64) -> Hsla {
    Hsla { h, s, l, a }
}

#[test]
fn test_rgba_to_hsla_parity_gray_has_no_hue_or_saturation() {
    assert_eq!(
        umt_rgba_to_hsla(input(100.0, 100.0, 100.0, None)).unwrap(),
        hsla(0.0, 0.0, 39.22, 1.0)
    );
}

#[test]
fn test_rgba_to_hsla_parity_red_is_max_with_green_at_or_above_blue() {
    assert_eq!(
        umt_rgba_to_hsla(input(214.0, 54.0, 63.0, None)).unwrap(),
        hsla(356.63, 66.12, 52.55, 1.0)
    );
    assert_eq!(
        umt_rgba_to_hsla(input(255.0, 255.0, 0.0, None)).unwrap(),
        hsla(60.0, 100.0, 50.0, 1.0)
    );
}

#[test]
fn test_rgba_to_hsla_parity_red_is_max_with_green_below_blue() {
    assert_eq!(
        umt_rgba_to_hsla(input(255.0, 0.0, 100.0, None)).unwrap(),
        hsla(336.47, 100.0, 50.0, 1.0)
    );
}

#[test]
fn test_rgba_to_hsla_parity_green_is_max() {
    assert_eq!(
        umt_rgba_to_hsla(input(161.0, 207.0, 143.0, None)).unwrap(),
        hsla(103.13, 40.0, 68.63, 1.0)
    );
    assert_eq!(
        umt_rgba_to_hsla(input(47.0, 143.0, 124.0, Some(0.4))).unwrap(),
        hsla(168.13, 50.53, 37.25, 0.4)
    );
}

#[test]
fn test_rgba_to_hsla_parity_blue_is_max() {
    assert_eq!(
        umt_rgba_to_hsla(input(61.0, 6.0, 166.0, None)).unwrap(),
        hsla(260.63, 93.02, 33.73, 1.0)
    );
    assert_eq!(
        umt_rgba_to_hsla(input(173.0, 216.0, 230.0, None)).unwrap(),
        hsla(194.74, 53.27, 79.02, 1.0)
    );
}

#[test]
fn test_rgba_to_hsla_parity_out_of_range_values_are_errors() {
    assert!(umt_rgba_to_hsla(input(-1.0, 0.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(256.0, 0.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, -1.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, 256.0, 0.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, 0.0, -1.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, 0.0, 256.0, None)).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, 0.0, 0.0, Some(-0.1))).is_err());
    assert!(umt_rgba_to_hsla(input(0.0, 0.0, 0.0, Some(1.1))).is_err());
}
