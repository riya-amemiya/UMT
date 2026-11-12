use umt_rust::time::umt_format_duration;

#[test]
fn test_returns_0s_for_zero() {
    assert_eq!(umt_format_duration(0.0), "0s");
    assert_eq!(umt_format_duration(-0.0), "0s");
}

#[test]
fn test_formats_single_units() {
    assert_eq!(umt_format_duration(45_000.0), "45s");
    assert_eq!(umt_format_duration(60_000.0), "1m");
    assert_eq!(umt_format_duration(3_600_000.0), "1h");
    assert_eq!(umt_format_duration(86_400_000.0), "1d");
    assert_eq!(umt_format_duration(250.0), "250ms");
}

#[test]
fn test_omits_zero_units() {
    assert_eq!(umt_format_duration(5_400_000.0), "1h 30m");
    assert_eq!(umt_format_duration(183_600_000.0), "2d 3h");
    assert_eq!(umt_format_duration(86_400_005.0), "1d 5ms");
}

#[test]
fn test_formats_every_unit() {
    assert_eq!(umt_format_duration(93_784_005.0), "1d 2h 3m 4s 5ms");
}

#[test]
fn test_does_not_cap_days() {
    assert_eq!(umt_format_duration(400.0 * 86_400_000.0), "400d");
}

#[test]
fn test_truncates_fractional_milliseconds() {
    assert_eq!(umt_format_duration(1_500.9), "1s 500ms");
    assert_eq!(umt_format_duration(0.5), "0s");
}

#[test]
fn test_keeps_leading_minus_for_negative() {
    assert_eq!(umt_format_duration(-90_000.0), "-1m 30s");
    assert_eq!(umt_format_duration(-5_400_000.0), "-1h 30m");
}

#[test]
fn test_returns_0s_without_sign_when_negative_truncates_to_zero() {
    assert_eq!(umt_format_duration(-0.5), "0s");
}
