//! Tests for the rgba_to_hexa module.

use umt_rust::color::{RgbaInput, umt_rgba_to_hexa};

#[test]
fn test_rgba_to_hexa_valid_conversions() {
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
fn test_rgba_to_hexa_invalid_values() {
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

use umt_rust::color::*;

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
