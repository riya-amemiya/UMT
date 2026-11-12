//! Parity tests for the cmyk_to_rgba module.

use umt_rust::color::{Rgba, umt_cmyk_to_rgba};

#[test]
fn test_cmyk_to_rgba_parity_rounds_like_typescript() {
    assert_eq!(
        umt_cmyk_to_rgba(60.0, 20.0, 10.0, 25.0, Some(0.5)),
        Rgba {
            r: 77.0,
            g: 153.0,
            b: 172.0,
            a: 0.5
        }
    );
    assert_eq!(
        umt_cmyk_to_rgba(0.0, 100.0, 25.0, 90.0, Some(0.25)),
        Rgba {
            r: 26.0,
            g: 0.0,
            b: 19.0,
            a: 0.25
        }
    );
    assert_eq!(
        umt_cmyk_to_rgba(0.0, 70.0, 20.0, 30.0, None),
        Rgba {
            r: 179.0,
            g: 54.0,
            b: 143.0,
            a: 1.0
        }
    );
}

#[test]
fn test_cmyk_to_rgba_parity_clamps_out_of_range_values() {
    assert_eq!(
        umt_cmyk_to_rgba(150.0, -20.0, 50.0, 0.0, Some(2.0)),
        Rgba {
            r: 0.0,
            g: 255.0,
            b: 128.0,
            a: 1.0
        }
    );
    assert_eq!(umt_cmyk_to_rgba(0.0, 0.0, 0.0, 0.0, Some(-1.0)).a, 0.0);
}

#[test]
fn test_cmyk_to_rgba_parity_propagates_nan() {
    let result = umt_cmyk_to_rgba(f64::NAN, 0.0, 0.0, 0.0, None);
    assert!(result.r.is_nan());
    assert_eq!(result.g, 255.0);
    assert_eq!(result.b, 255.0);
    assert_eq!(result.a, 1.0);
}
