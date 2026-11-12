use umt_rust::color::{RgbaInput, umt_get_contrast_ratio};

fn rgb(r: f64, g: f64, b: f64) -> RgbaInput {
    RgbaInput { r, g, b, a: None }
}

#[test]
fn test_black_and_white() {
    let ratio = umt_get_contrast_ratio(
        rgb(0.0, 0.0, 0.0),
        RgbaInput {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: Some(1.0),
        },
    )
    .unwrap();
    assert!((ratio - 21.0).abs() < 1e-12);
}

#[test]
fn test_argument_order() {
    let white = rgb(255.0, 255.0, 255.0);
    let gray = rgb(119.0, 119.0, 119.0);
    let ratio = umt_get_contrast_ratio(white, gray).unwrap();
    assert!((ratio - 4.478_089_453_577_214).abs() < 1e-12);
    assert_eq!(umt_get_contrast_ratio(gray, white).unwrap(), ratio);
}

#[test]
fn test_identical_colors() {
    let white = rgb(255.0, 255.0, 255.0);
    let black = rgb(0.0, 0.0, 0.0);
    assert_eq!(umt_get_contrast_ratio(white, white).unwrap(), 1.0);
    assert_eq!(umt_get_contrast_ratio(black, black).unwrap(), 1.0);
}

#[test]
fn test_saturated_colors() {
    let ratio = umt_get_contrast_ratio(rgb(255.0, 0.0, 0.0), rgb(0.0, 0.0, 255.0)).unwrap();
    assert!((ratio - 2.148_936_170_212_766).abs() < 1e-12);
}

#[test]
fn test_invalid_values() {
    let white = rgb(255.0, 255.0, 255.0);
    assert!(umt_get_contrast_ratio(rgb(256.0, 0.0, 0.0), white).is_err());
    assert!(
        umt_get_contrast_ratio(
            white,
            RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: Some(2.0)
            }
        )
        .is_err()
    );
}
