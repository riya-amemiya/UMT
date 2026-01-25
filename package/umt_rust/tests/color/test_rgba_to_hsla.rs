//! Tests for the rgba_to_hsla module.

use umt_rust::color::{Hsla, RgbaInput, umt_rgba_to_hsla};

#[test]
fn test_rgba_to_hsla_invalid_values() {
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: -1.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 256.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: -1.0,
            b: 0.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: 256.0,
            b: 0.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: 0.0,
            b: -1.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 256.0,
            a: Some(1.0)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: Some(-0.1)
        })
        .is_err()
    );
    assert!(
        umt_rgba_to_hsla(RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.1)
        })
        .is_err()
    );
}

#[test]
fn test_rgba_to_hsla_error_message() {
    let err = umt_rgba_to_hsla(RgbaInput {
        r: -1.0,
        g: 0.0,
        b: 0.0,
        a: Some(1.0),
    })
    .unwrap_err();
    assert_eq!(err.message, "Invalid rgba value");
}

#[test]
fn test_rgba_to_hsla_edge_cases() {
    // Black
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0
        }
    );

    // White
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 255.0,
        g: 255.0,
        b: 255.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 100.0,
            a: 1.0
        }
    );
}

#[test]
fn test_rgba_to_hsla_conversions() {
    // Gray
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 100.0,
        g: 100.0,
        b: 100.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 39.22,
            a: 1.0
        }
    );

    // Red
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 255.0,
        g: 0.0,
        b: 0.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 0.0,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );

    // Green
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 0.0,
        g: 255.0,
        b: 0.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 120.0,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );

    // Blue
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 0.0,
        g: 0.0,
        b: 255.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 240.0,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );

    // Red with alpha 0.5
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 255.0,
        g: 0.0,
        b: 0.0,
        a: Some(0.5),
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 0.0,
            s: 100.0,
            l: 50.0,
            a: 0.5
        }
    );

    // Yellow
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 255.0,
        g: 255.0,
        b: 0.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 60.0,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );

    // Blue with alpha 0.7
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 0.0,
        g: 0.0,
        b: 255.0,
        a: Some(0.7),
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 240.0,
            s: 100.0,
            l: 50.0,
            a: 0.7
        }
    );

    // Light blue
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 173.0,
        g: 216.0,
        b: 230.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 194.74,
            s: 53.27,
            l: 79.02,
            a: 1.0
        }
    );

    // Pink
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 255.0,
        g: 0.0,
        b: 100.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 336.47,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );

    // Purple
    let result = umt_rgba_to_hsla(RgbaInput {
        r: 100.0,
        g: 0.0,
        b: 255.0,
        a: None,
    })
    .unwrap();
    assert_eq!(
        result,
        Hsla {
            h: 263.53,
            s: 100.0,
            l: 50.0,
            a: 1.0
        }
    );
}
