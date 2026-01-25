use umt_rust::ip::long_to_ip;

// Test valid 32-bit integers
#[test]
fn test_long_to_ip_common_private_network() {
    assert_eq!(long_to_ip(0xc0_a8_00_01), "192.168.0.1");
}

#[test]
fn test_long_to_ip_class_b_start() {
    assert_eq!(long_to_ip(0x80_00_00_01), "128.0.0.1");
}

#[test]
fn test_long_to_ip_class_a_private() {
    assert_eq!(long_to_ip(0x0a_00_00_01), "10.0.0.1");
}

#[test]
fn test_long_to_ip_class_b_private() {
    assert_eq!(long_to_ip(0xac_10_00_01), "172.16.0.1");
}

#[test]
fn test_long_to_ip_maximum_value() {
    assert_eq!(long_to_ip(0xff_ff_ff_ff), "255.255.255.255");
}

#[test]
fn test_long_to_ip_minimum_value() {
    assert_eq!(long_to_ip(0x00_00_00_00), "0.0.0.0");
}

#[test]
fn test_long_to_ip_localhost() {
    assert_eq!(long_to_ip(0x7f_00_00_01), "127.0.0.1");
}

#[test]
fn test_long_to_ip_simple_incremental() {
    assert_eq!(long_to_ip(0x01_02_03_04), "1.2.3.4");
}

// Note: In Rust, the long_to_ip function takes a u32, so it cannot receive
// invalid inputs like undefined, null, NaN, Infinity, negative numbers,
// numbers larger than u32::MAX, non-integers, or strings.
// The type system prevents these at compile time.
// The TypeScript tests for these cases are not applicable to the Rust implementation.
