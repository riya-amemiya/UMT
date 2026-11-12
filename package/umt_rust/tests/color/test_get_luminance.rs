use umt_rust::color::{RgbaError, RgbaInput, umt_get_luminance};

fn rgb(r: f64, g: f64, b: f64) -> RgbaInput {
    RgbaInput { r, g, b, a: None }
}

#[test]
fn test_relative_luminance() {
    for (input, expected) in [
        (rgb(255.0, 255.0, 255.0), 1.0),
        (rgb(0.0, 0.0, 0.0), 0.0),
        (rgb(255.0, 0.0, 0.0), 0.2126),
        (rgb(0.0, 255.0, 0.0), 0.7152),
        (rgb(0.0, 0.0, 255.0), 0.0722),
        (rgb(128.0, 128.0, 128.0), 0.215_860_500_113_899_23),
        (rgb(119.0, 119.0, 119.0), 0.184_474_994_500_441),
    ] {
        let actual = umt_get_luminance(input).unwrap();
        assert!((actual - expected).abs() < 1e-12, "{:?} {}", input, actual);
    }
}

#[test]
fn test_linear_segment() {
    let actual = umt_get_luminance(rgb(10.0, 10.0, 10.0)).unwrap();
    assert!((actual - 10.0 / 255.0 / 12.92).abs() < 1e-15);
    let actual = umt_get_luminance(rgb(1.0, 2.0, 3.0)).unwrap();
    let expected = (0.2126 * 1.0 + 0.7152 * 2.0 + 0.0722 * 3.0) / 255.0 / 12.92;
    assert!((actual - expected).abs() < 1e-15);
}

#[test]
fn test_ignores_alpha() {
    assert_eq!(
        umt_get_luminance(RgbaInput {
            r: 128.0,
            g: 64.0,
            b: 32.0,
            a: Some(0.0)
        }),
        umt_get_luminance(rgb(128.0, 64.0, 32.0))
    );
}

#[test]
fn test_invalid_values() {
    for input in [
        rgb(-1.0, 0.0, 0.0),
        rgb(256.0, 0.0, 0.0),
        rgb(0.0, -1.0, 0.0),
        rgb(0.0, 256.0, 0.0),
        rgb(0.0, 0.0, -1.0),
        rgb(0.0, 0.0, 256.0),
        rgb(f64::NAN, 0.0, 0.0),
        RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: Some(-0.1),
        },
        RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.1),
        },
    ] {
        assert_eq!(
            umt_get_luminance(input),
            Err(RgbaError {
                message: "Invalid rgba value".to_string()
            }),
            "{:?}",
            input
        );
    }
}
