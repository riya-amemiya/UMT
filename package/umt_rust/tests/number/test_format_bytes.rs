use umt_rust::number::{umt_format_bytes, umt_format_bytes_default};

#[test]
fn test_formats_zero_as_bytes() {
    assert_eq!(umt_format_bytes_default(0.0), "0 B");
}

#[test]
fn test_keeps_values_below_1024_in_bytes() {
    assert_eq!(umt_format_bytes_default(1.0), "1 B");
    assert_eq!(umt_format_bytes_default(512.0), "512 B");
    assert_eq!(umt_format_bytes_default(1023.0), "1023 B");
}

#[test]
fn test_keeps_fractional_bytes_below_1_in_bytes() {
    assert_eq!(umt_format_bytes_default(0.5), "0.5 B");
}

#[test]
fn test_uses_each_unit_at_its_boundary() {
    assert_eq!(umt_format_bytes_default(1024.0), "1 KB");
    assert_eq!(umt_format_bytes_default(1024_f64.powi(2)), "1 MB");
    assert_eq!(umt_format_bytes_default(1024_f64.powi(3)), "1 GB");
    assert_eq!(umt_format_bytes_default(1024_f64.powi(4)), "1 TB");
    assert_eq!(umt_format_bytes_default(1024_f64.powi(5)), "1 PB");
}

#[test]
fn test_stays_in_pb_for_1024_pb_or_more() {
    assert_eq!(umt_format_bytes_default(1024_f64.powi(6)), "1024 PB");
}

#[test]
fn test_rounds_to_two_decimals_by_default() {
    assert_eq!(umt_format_bytes_default(1536.0), "1.5 KB");
    assert_eq!(umt_format_bytes_default(1_234_567.0), "1.18 MB");
    assert_eq!(umt_format_bytes_default(1_048_575.0), "1024 KB");
}

#[test]
fn test_respects_custom_decimals() {
    assert_eq!(umt_format_bytes(1_234_567.0, 0), "1 MB");
    assert_eq!(umt_format_bytes(1_234_567.0, 1), "1.2 MB");
    assert_eq!(umt_format_bytes(1_234_567.0, 4), "1.1774 MB");
}

#[test]
fn test_formats_negative_values() {
    assert_eq!(umt_format_bytes_default(-2048.0), "-2 KB");
    assert_eq!(umt_format_bytes_default(-1536.0), "-1.5 KB");
}

#[test]
fn test_does_not_print_negative_zero() {
    assert_eq!(umt_format_bytes_default(-0.0), "0 B");
    assert_eq!(umt_format_bytes_default(-0.001), "0 B");
}
