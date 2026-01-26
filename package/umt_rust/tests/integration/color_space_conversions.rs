//! Integration tests for Color conversion functions
//!
//! Tests the interaction between different color space conversions:
//! - Round-trip conversions (RGB -> HSL -> RGB)
//! - Multi-step color transformations
//! - Accuracy of conversions across color spaces

use umt_rust::color::{
    RgbaInput, umt_cmyk_to_rgba, umt_hexa_to_rgba, umt_hsla_to_rgba, umt_rgba_to_cmyk,
    umt_rgba_to_hexa, umt_rgba_to_hsla,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn close_to(actual: f64, expected: f64, precision: i32) -> bool {
        let factor = 10_f64.powi(precision);
        (actual * factor).round() == (expected * factor).round()
    }

    #[test]
    fn should_perform_roundtrip_conversion_rgba_to_hsla_to_rgba() {
        let test_colors = vec![
            RgbaInput {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0),
            }, // Red
            RgbaInput {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: Some(0.5),
            }, // Green with alpha
            RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: Some(1.0),
            }, // Blue
            RgbaInput {
                r: 128.0,
                g: 128.0,
                b: 128.0,
                a: Some(1.0),
            }, // Gray
            RgbaInput {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: Some(0.8),
            }, // White with alpha
        ];

        for original in test_colors {
            let hsla = umt_rgba_to_hsla(original.clone()).unwrap();
            let converted = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();

            assert!(
                close_to(converted.r, original.r, 0),
                "Red mismatch: {} vs {}",
                converted.r,
                original.r
            );
            assert!(
                close_to(converted.g, original.g, 0),
                "Green mismatch: {} vs {}",
                converted.g,
                original.g
            );
            assert!(
                close_to(converted.b, original.b, 0),
                "Blue mismatch: {} vs {}",
                converted.b,
                original.b
            );
            assert!(
                close_to(converted.a, original.a.unwrap_or(1.0), 2),
                "Alpha mismatch: {} vs {}",
                converted.a,
                original.a.unwrap_or(1.0)
            );
        }
    }

    #[test]
    fn should_perform_roundtrip_conversion_rgba_to_cmyk_to_rgba() {
        let test_colors = vec![
            RgbaInput {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0),
            }, // Red
            RgbaInput {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: Some(1.0),
            }, // Green
            RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 255.0,
                a: Some(1.0),
            }, // Blue
            RgbaInput {
                r: 100.0,
                g: 150.0,
                b: 200.0,
                a: Some(0.7),
            }, // Custom color with alpha
        ];

        for original in test_colors {
            let cmyk = umt_rgba_to_cmyk(original.clone()).unwrap();
            let converted = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));

            assert!(
                close_to(converted.r, original.r, 0),
                "Red mismatch: {} vs {}",
                converted.r,
                original.r
            );
            assert!(
                close_to(converted.g, original.g, 0),
                "Green mismatch: {} vs {}",
                converted.g,
                original.g
            );
            assert!(
                close_to(converted.b, original.b, 0),
                "Blue mismatch: {} vs {}",
                converted.b,
                original.b
            );
            assert!(
                close_to(converted.a, original.a.unwrap_or(1.0), 2),
                "Alpha mismatch: {} vs {}",
                converted.a,
                original.a.unwrap_or(1.0)
            );
        }
    }

    #[test]
    fn should_perform_roundtrip_conversion_rgba_to_hex_to_rgba() {
        let test_colors = vec![
            RgbaInput {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0),
            }, // Red
            RgbaInput {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: Some(0.5),
            }, // Green with alpha
            RgbaInput {
                r: 128.0,
                g: 64.0,
                b: 192.0,
                a: Some(0.75),
            }, // Purple with alpha
        ];

        for original in test_colors {
            let hex = umt_rgba_to_hexa(original.clone()).unwrap();
            let converted = umt_hexa_to_rgba(&hex).unwrap();

            assert_eq!(converted.r as i32, original.r as i32);
            assert_eq!(converted.g as i32, original.g as i32);
            assert_eq!(converted.b as i32, original.b as i32);
            assert!(
                close_to(converted.a, original.a.unwrap_or(1.0), 2),
                "Alpha mismatch: {} vs {}",
                converted.a,
                original.a.unwrap_or(1.0)
            );
        }
    }

    #[test]
    fn should_handle_multi_step_color_transformations() {
        let original_rgba = RgbaInput {
            r: 75.0,
            g: 150.0,
            b: 225.0,
            a: Some(0.9),
        };

        // RGBA -> HSLA
        let hsla = umt_rgba_to_hsla(original_rgba.clone()).unwrap();

        // HSLA -> RGBA -> CMYK
        let rgba_from_hsla = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();
        let rgba_input = RgbaInput {
            r: rgba_from_hsla.r,
            g: rgba_from_hsla.g,
            b: rgba_from_hsla.b,
            a: Some(rgba_from_hsla.a),
        };
        let cmyk = umt_rgba_to_cmyk(rgba_input).unwrap();

        // CMYK -> RGBA -> HEX
        let rgba_from_cmyk = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));
        let rgba_input2 = RgbaInput {
            r: rgba_from_cmyk.r,
            g: rgba_from_cmyk.g,
            b: rgba_from_cmyk.b,
            a: Some(rgba_from_cmyk.a),
        };
        let hex = umt_rgba_to_hexa(rgba_input2).unwrap();

        // HEX -> RGBA
        let final_rgba = umt_hexa_to_rgba(&hex).unwrap();

        assert!(
            close_to(final_rgba.r, original_rgba.r, 0),
            "Final red mismatch: {} vs {}",
            final_rgba.r,
            original_rgba.r
        );
        assert!(
            close_to(final_rgba.g, original_rgba.g, 0),
            "Final green mismatch: {} vs {}",
            final_rgba.g,
            original_rgba.g
        );
        assert!(
            close_to(final_rgba.b, original_rgba.b, 0),
            "Final blue mismatch: {} vs {}",
            final_rgba.b,
            original_rgba.b
        );
        assert!(
            close_to(final_rgba.a, original_rgba.a.unwrap_or(1.0), 2),
            "Final alpha mismatch: {} vs {}",
            final_rgba.a,
            original_rgba.a.unwrap_or(1.0)
        );
    }

    #[test]
    fn should_handle_grayscale_conversions_across_color_spaces() {
        let gray_levels = [0.0, 64.0, 128.0, 192.0, 255.0];

        for gray in gray_levels {
            let rgba = RgbaInput {
                r: gray,
                g: gray,
                b: gray,
                a: Some(1.0),
            };

            // Test HSLA conversion
            let hsla = umt_rgba_to_hsla(rgba.clone()).unwrap();
            assert_eq!(hsla.s, 0.0, "Saturation should be 0 for gray");

            let rgba_from_hsla = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();
            assert!(
                close_to(rgba_from_hsla.r, gray, 0),
                "HSLA roundtrip red mismatch: {} vs {}",
                rgba_from_hsla.r,
                gray
            );
            assert!(
                close_to(rgba_from_hsla.g, gray, 0),
                "HSLA roundtrip green mismatch"
            );
            assert!(
                close_to(rgba_from_hsla.b, gray, 0),
                "HSLA roundtrip blue mismatch"
            );

            // Test CMYK conversion
            let cmyk = umt_rgba_to_cmyk(rgba.clone()).unwrap();
            let rgba_from_cmyk = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));
            assert!(
                close_to(rgba_from_cmyk.r, gray, 0),
                "CMYK roundtrip red mismatch"
            );
            assert!(
                close_to(rgba_from_cmyk.g, gray, 0),
                "CMYK roundtrip green mismatch"
            );
            assert!(
                close_to(rgba_from_cmyk.b, gray, 0),
                "CMYK roundtrip blue mismatch"
            );
        }
    }

    #[test]
    fn should_preserve_alpha_channel_through_conversions() {
        let alpha_values = [0.0, 0.25, 0.5, 0.75, 1.0];
        let base_color = (100.0, 150.0, 200.0);

        for alpha in alpha_values {
            let rgba = RgbaInput {
                r: base_color.0,
                g: base_color.1,
                b: base_color.2,
                a: Some(alpha),
            };

            // Test through HSLA
            let hsla = umt_rgba_to_hsla(rgba.clone()).unwrap();
            assert!(
                close_to(hsla.a, alpha, 2),
                "HSLA alpha mismatch: {} vs {}",
                hsla.a,
                alpha
            );

            // Test through CMYK
            let cmyk = umt_rgba_to_cmyk(rgba.clone()).unwrap();
            assert!(
                close_to(cmyk.a, alpha, 2),
                "CMYK alpha mismatch: {} vs {}",
                cmyk.a,
                alpha
            );

            // Test through HEX
            let hex = umt_rgba_to_hexa(rgba.clone()).unwrap();
            let rgba_from_hex = umt_hexa_to_rgba(&hex).unwrap();
            assert!(
                close_to(rgba_from_hex.a, alpha, 2),
                "HEX alpha mismatch: {} vs {}",
                rgba_from_hex.a,
                alpha
            );
        }
    }

    #[test]
    fn should_handle_edge_cases_in_color_conversions() {
        let edge_cases = vec![
            RgbaInput {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: Some(1.0),
            }, // Black
            RgbaInput {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: Some(1.0),
            }, // White
            RgbaInput {
                r: 255.0,
                g: 0.0,
                b: 255.0,
                a: Some(0.0),
            }, // Magenta with 0 alpha
        ];

        for color in edge_cases {
            // Test all conversion paths
            let hsla = umt_rgba_to_hsla(color.clone()).unwrap();
            let cmyk = umt_rgba_to_cmyk(color.clone()).unwrap();
            let hex = umt_rgba_to_hexa(color.clone()).unwrap();

            let from_hsla = umt_hsla_to_rgba(hsla.h, hsla.s, hsla.l, Some(hsla.a)).unwrap();
            let from_cmyk = umt_cmyk_to_rgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, Some(cmyk.a));
            let from_hex = umt_hexa_to_rgba(&hex).unwrap();

            // All conversions should yield similar results
            for converted in [from_hsla, from_cmyk, from_hex] {
                assert!(
                    close_to(converted.r, color.r, 0),
                    "Edge case red mismatch: {} vs {}",
                    converted.r,
                    color.r
                );
                assert!(
                    close_to(converted.g, color.g, 0),
                    "Edge case green mismatch: {} vs {}",
                    converted.g,
                    color.g
                );
                assert!(
                    close_to(converted.b, color.b, 0),
                    "Edge case blue mismatch: {} vs {}",
                    converted.b,
                    color.b
                );
                assert!(
                    close_to(converted.a, color.a.unwrap_or(1.0), 2),
                    "Edge case alpha mismatch: {} vs {}",
                    converted.a,
                    color.a.unwrap_or(1.0)
                );
            }
        }
    }

    #[test]
    fn should_handle_color_mixing_through_conversions() {
        let color1 = RgbaInput {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: Some(1.0),
        }; // Red
        let color2 = RgbaInput {
            r: 0.0,
            g: 0.0,
            b: 255.0,
            a: Some(1.0),
        }; // Blue

        // Convert to HSLA for mixing
        let hsla1 = umt_rgba_to_hsla(color1).unwrap();
        let hsla2 = umt_rgba_to_hsla(color2).unwrap();

        // Mix in HSLA space
        let mixed_h = (hsla1.h + hsla2.h) / 2.0;
        let mixed_s = (hsla1.s + hsla2.s) / 2.0;
        let mixed_l = (hsla1.l + hsla2.l) / 2.0;
        let mixed_a = (hsla1.a + hsla2.a) / 2.0;

        let mixed_rgba = umt_hsla_to_rgba(mixed_h, mixed_s, mixed_l, Some(mixed_a)).unwrap();
        let mixed_input = RgbaInput {
            r: mixed_rgba.r,
            g: mixed_rgba.g,
            b: mixed_rgba.b,
            a: Some(mixed_rgba.a),
        };
        let mixed_hex = umt_rgba_to_hexa(mixed_input).unwrap();

        // Verify the mixed color has valid properties
        assert!(
            mixed_rgba.r >= 0.0,
            "Mixed red should be >= 0: {}",
            mixed_rgba.r
        );
        assert!(
            mixed_rgba.b >= 0.0,
            "Mixed blue should be >= 0: {}",
            mixed_rgba.b
        );

        // Check hex format (should be #RRGGBBAA)
        let is_valid_hex = mixed_hex.len() == 9
            && mixed_hex.starts_with('#')
            && mixed_hex[1..].chars().all(|c| c.is_ascii_hexdigit());
        assert!(
            is_valid_hex,
            "Hex should match pattern: {}",
            mixed_hex
        );
    }
}
