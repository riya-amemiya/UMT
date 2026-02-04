use umt_rust::color::{
    umt_rgba_to_hsla, umt_hsla_to_rgba,
    umt_rgba_to_cmyk, umt_cmyk_to_rgba,
    umt_rgba_to_hexa, umt_hexa_to_rgba,
    RgbaInput,
};

#[test]
fn test_round_trip_rgba_hsla_rgba() {
    let test_colors = vec![
        (255.0, 0.0, 0.0, 1.0),       // Red
        (0.0, 255.0, 0.0, 0.5),       // Green with alpha
        (0.0, 0.0, 255.0, 1.0),       // Blue
        (128.0, 128.0, 128.0, 1.0),   // Gray
        (255.0, 255.0, 255.0, 0.8),   // White with alpha
    ];

    for (r, g, b, a) in test_colors {
        let input = RgbaInput { r, g, b, a: Some(a) };
        let hsla = umt_rgba_to_hsla(input).unwrap();
        let rgba = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();

        assert!((rgba.r - r).abs() <= 1.0);
        assert!((rgba.g - g).abs() <= 1.0);
        assert!((rgba.b - b).abs() <= 1.0);
        assert!((rgba.a - a).abs() < 0.01);
    }
}

#[test]
fn test_round_trip_rgba_cmyk_rgba() {
    let test_colors = vec![
        (255.0, 0.0, 0.0, 1.0),
        (0.0, 255.0, 0.0, 1.0),
        (0.0, 0.0, 255.0, 1.0),
        (100.0, 150.0, 200.0, 0.7),
    ];

    for (r, g, b, a) in test_colors {
        let input = RgbaInput { r, g, b, a: Some(a) };
        let cmyk = umt_rgba_to_cmyk(input).unwrap();
        let rgba = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));

        assert!((rgba.r - r).abs() <= 1.0);
        assert!((rgba.g - g).abs() <= 1.0);
        assert!((rgba.b - b).abs() <= 1.0);
        assert!((rgba.a - a).abs() < 0.01);
    }
}

#[test]
fn test_round_trip_rgba_hex_rgba() {
    let test_colors = vec![
        (255.0, 0.0, 0.0, 1.0),
        (0.0, 255.0, 0.0, 0.5),
        (128.0, 64.0, 192.0, 0.75),
    ];

    for (r, g, b, a) in test_colors {
        let input = RgbaInput { r, g, b, a: Some(a) };
        let hex = umt_rgba_to_hexa(input).unwrap();
        let rgba = umt_hexa_to_rgba(&hex).unwrap();

        // Hex conversion can lose precision on floats, but integers should be close
        assert!((rgba.r - r).abs() < 1.0);
        assert!((rgba.g - g).abs() < 1.0);
        assert!((rgba.b - b).abs() < 1.0);
        // Alpha in hex is 2 digits, so 1/255 precision approx 0.004
        assert!((rgba.a - a).abs() < 0.01);
    }
}

#[test]
fn test_multi_step_transformations() {
    let (r, g, b, a) = (75.0, 150.0, 225.0, 0.9);

    // RGBA -> HSLA
    let input = RgbaInput { r, g, b, a: Some(a) };
    let hsla = umt_rgba_to_hsla(input).unwrap();

    // HSLA -> RGBA -> CMYK
    let rgba_from_hsla = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();
    let input_cmyk = RgbaInput {
        r: rgba_from_hsla.r,
        g: rgba_from_hsla.g,
        b: rgba_from_hsla.b,
        a: Some(rgba_from_hsla.a),
    };
    let cmyk = umt_rgba_to_cmyk(input_cmyk).unwrap();

    // CMYK -> RGBA -> HEX
    let rgba_from_cmyk = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));
    let input_hex = RgbaInput {
        r: rgba_from_cmyk.r,
        g: rgba_from_cmyk.g,
        b: rgba_from_cmyk.b,
        a: Some(rgba_from_cmyk.a),
    };
    let hex = umt_rgba_to_hexa(input_hex).unwrap();

    // HEX -> RGBA
    let final_rgba = umt_hexa_to_rgba(&hex).unwrap();

    assert!((final_rgba.r - r).abs() <= 2.0); // Slight precision loss acceptable
    assert!((final_rgba.g - g).abs() <= 2.0);
    assert!((final_rgba.b - b).abs() <= 2.0);
    assert!((final_rgba.a - a).abs() < 0.02);
}
