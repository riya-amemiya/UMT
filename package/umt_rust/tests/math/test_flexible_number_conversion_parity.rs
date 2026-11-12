use umt_rust::math::umt_flexible_number_conversion;

#[test]
fn test_flexible_rust_only_spellings_are_nan() {
    assert!(umt_flexible_number_conversion("inf").is_nan());
    assert!(umt_flexible_number_conversion("-inf").is_nan());
    assert!(umt_flexible_number_conversion("nan").is_nan());
    assert!(umt_flexible_number_conversion("infinityx").is_nan());
}

#[test]
fn test_flexible_signed_radix_digits_are_nan() {
    assert!(umt_flexible_number_conversion("0x-ff").is_nan());
    assert!(umt_flexible_number_conversion("0b+1").is_nan());
    assert!(umt_flexible_number_conversion("0o-7").is_nan());
}

#[test]
fn test_flexible_invalid_radix_digits_are_nan() {
    assert!(umt_flexible_number_conversion("0xZZ").is_nan());
    assert!(umt_flexible_number_conversion("0x").is_nan());
    assert!(umt_flexible_number_conversion("0o9").is_nan());
    assert!(umt_flexible_number_conversion("0b2").is_nan());
}

#[test]
fn test_flexible_valid_radix_digits() {
    assert_eq!(umt_flexible_number_conversion("0XfF"), 255.0);
    assert_eq!(umt_flexible_number_conversion("0o17"), 15.0);
    assert_eq!(umt_flexible_number_conversion("0b101"), 5.0);
}

#[test]
fn test_flexible_dangling_exponent_is_ignored() {
    assert_eq!(umt_flexible_number_conversion("1e"), 1.0);
    assert_eq!(umt_flexible_number_conversion("1e+"), 1.0);
    assert_eq!(umt_flexible_number_conversion("2ex"), 2.0);
}

#[test]
fn test_flexible_exponent_before_trailing_text() {
    assert_eq!(umt_flexible_number_conversion("1e3x"), 1000.0);
    assert_eq!(umt_flexible_number_conversion("1e+2z"), 100.0);
    assert_eq!(umt_flexible_number_conversion("1.5E-1"), 0.15);
}

#[test]
fn test_flexible_sign_and_decimal_point_forms() {
    assert_eq!(umt_flexible_number_conversion("+42px"), 42.0);
    assert_eq!(umt_flexible_number_conversion("-42px"), -42.0);
    assert_eq!(umt_flexible_number_conversion("-.5"), -0.5);
    assert_eq!(umt_flexible_number_conversion("5."), 5.0);
    assert_eq!(umt_flexible_number_conversion("1.5.5"), 1.5);
    assert_eq!(umt_flexible_number_conversion("1_000"), 1.0);
}

#[test]
fn test_flexible_no_digits_is_nan() {
    assert!(umt_flexible_number_conversion(".").is_nan());
    assert!(umt_flexible_number_conversion("-").is_nan());
    assert!(umt_flexible_number_conversion("-.x").is_nan());
    assert!(umt_flexible_number_conversion("--1").is_nan());
    assert!(umt_flexible_number_conversion("abc").is_nan());
}

#[test]
fn test_flexible_empty_and_infinity() {
    assert_eq!(umt_flexible_number_conversion(""), 0.0);
    assert!(umt_flexible_number_conversion("   ").is_nan());
    assert_eq!(umt_flexible_number_conversion(" +Infinity "), f64::INFINITY);
    assert_eq!(
        umt_flexible_number_conversion("-INFINITY"),
        f64::NEG_INFINITY
    );
}

#[test]
fn test_flexible_javascript_whitespace_set() {
    assert_eq!(umt_flexible_number_conversion("\u{feff}1\u{a0}"), 1.0);
    assert_eq!(umt_flexible_number_conversion("\t\n 7 \u{3000}"), 7.0);
    assert!(umt_flexible_number_conversion("\u{85}1").is_nan());
    assert!(umt_flexible_number_conversion("\u{1c}1").is_nan());
}

#[test]
fn test_flexible_radix_digits_beyond_i64() {
    assert_eq!(
        umt_flexible_number_conversion("0xffffffffffffffff"),
        18446744073709551615.0
    );
    assert_eq!(
        umt_flexible_number_conversion(&format!("0x1{}", "0".repeat(40))),
        16_f64.powi(40)
    );
    assert_eq!(
        umt_flexible_number_conversion(&format!("0b1{}", "0".repeat(130))),
        2_f64.powi(130)
    );
}
