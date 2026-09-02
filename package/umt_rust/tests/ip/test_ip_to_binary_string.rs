use umt_rust::ip::ip_to_binary_string;
use umt_rust::ip::ip_to_long;
use umt_rust::ip::long_to_ip;

// Test valid IP addresses
#[test]
fn test_ip_to_binary_string_common_private_network() {
    assert_eq!(
        ip_to_binary_string("192.168.0.1").unwrap(),
        "11000000101010000000000000000001"
    );
}

#[test]
fn test_ip_to_binary_string_all_zeros() {
    assert_eq!(
        ip_to_binary_string("0.0.0.0").unwrap(),
        "00000000000000000000000000000000"
    );
}

#[test]
fn test_ip_to_binary_string_all_ones() {
    assert_eq!(
        ip_to_binary_string("255.255.255.255").unwrap(),
        "11111111111111111111111111111111"
    );
}

#[test]
fn test_ip_to_binary_string_single_digit_octets() {
    assert_eq!(
        ip_to_binary_string("1.2.3.4").unwrap(),
        "00000001000000100000001100000100"
    );
}

#[test]
fn test_ip_to_binary_string_class_a_private() {
    assert_eq!(
        ip_to_binary_string("10.0.0.1").unwrap(),
        "00001010000000000000000000000001"
    );
}

#[test]
fn test_ip_to_binary_string_class_b_private() {
    assert_eq!(
        ip_to_binary_string("172.16.0.1").unwrap(),
        "10101100000100000000000000000001"
    );
}

#[test]
fn test_ip_to_binary_string_localhost() {
    assert_eq!(
        ip_to_binary_string("127.0.0.1").unwrap(),
        "01111111000000000000000000000001"
    );
}

#[test]
fn test_ip_to_binary_string_link_local() {
    assert_eq!(
        ip_to_binary_string("169.254.0.1").unwrap(),
        "10101001111111100000000000000001"
    );
}

// Test invalid IP addresses
#[test]
fn test_ip_to_binary_string_empty_string() {
    let result = ip_to_binary_string("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "IP address is required");
}

#[test]
fn test_ip_to_binary_string_incomplete_ip() {
    let result = ip_to_binary_string("192.168");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_non_numeric_octets() {
    let result = ip_to_binary_string("a.b.c.d");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_octet_too_large() {
    let result = ip_to_binary_string("256.1.2.3");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_negative_octet() {
    let result = ip_to_binary_string("-1.1.1.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_too_many_octets() {
    let result = ip_to_binary_string("1.2.3.4.5");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_too_few_octets() {
    let result = ip_to_binary_string("192.168.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_extra_octet() {
    let result = ip_to_binary_string("192.168.1.1.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_trailing_dot() {
    let result = ip_to_binary_string("192.168.1.");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_double_dot() {
    let result = ip_to_binary_string("192.168..1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_leading_dot() {
    let result = ip_to_binary_string(".192.168.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_comma_separator() {
    let result = ip_to_binary_string("192,168,1,1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_trailing_dot_after_valid() {
    let result = ip_to_binary_string("192.168.1.1.");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_plus_sign() {
    let result = ip_to_binary_string("192.168.1.+1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_all_octets_too_large() {
    let result = ip_to_binary_string("256.256.256.256");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_very_large_octets() {
    let result = ip_to_binary_string("999.999.999.999");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

// Test edge cases
#[test]
fn test_ip_to_binary_string_boundary_all_zeros() {
    assert_eq!(
        ip_to_binary_string("0.0.0.0").unwrap(),
        "00000000000000000000000000000000"
    );
}

#[test]
fn test_ip_to_binary_string_boundary_all_ones_same() {
    assert_eq!(
        ip_to_binary_string("1.1.1.1").unwrap(),
        "00000001000000010000000100000001"
    );
}

#[test]
fn test_ip_to_binary_string_boundary_max() {
    assert_eq!(
        ip_to_binary_string("255.255.255.255").unwrap(),
        "11111111111111111111111111111111"
    );
}

#[test]
fn test_ip_to_binary_string_boundary_128_first_octet() {
    assert_eq!(
        ip_to_binary_string("128.0.0.0").unwrap(),
        "10000000000000000000000000000000"
    );
}

#[test]
fn test_ip_to_binary_string_boundary_alternating() {
    assert_eq!(
        ip_to_binary_string("0.255.0.255").unwrap(),
        "00000000111111110000000011111111"
    );
}

// Test invalid formats with leading zeros and other edge cases
#[test]
fn test_ip_to_binary_string_just_digits() {
    let result = ip_to_binary_string("0000");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_two_octets_zeros() {
    let result = ip_to_binary_string("00.00");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_three_octets() {
    let result = ip_to_binary_string("0.0.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_leading_zero_third_octet() {
    let result = ip_to_binary_string("192.168.01.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_leading_zero_fourth_octet() {
    let result = ip_to_binary_string("192.168.1.01");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_all_leading_zeros() {
    let result = ip_to_binary_string("010.020.030.040");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_binary_string_matches_ip_to_long_bits() {
    let ips = [
        "192.168.0.1",
        "0.0.0.0",
        "255.255.255.255",
        "1.2.3.4",
        "10.0.0.1",
        "172.16.0.1",
        "127.0.0.1",
        "169.254.0.1",
        "8.8.8.8",
        "128.0.0.0",
        "0.255.0.255",
        "192.168.1.1",
    ];
    for ip in ips {
        let binary = ip_to_binary_string(ip).unwrap();
        let from_long = format!("{:032b}", ip_to_long(ip).unwrap());
        assert_eq!(binary, from_long);
        assert_eq!(binary.len(), 32);
    }
}

#[test]
fn test_ip_to_binary_string_roundtrip_long_to_ip() {
    let longs = [
        0u32,
        1,
        0x7f_00_00_01,
        0xc0_a8_00_01,
        0xff_ff_ff_ff,
        0x01_02_03_04,
        0x0a_00_00_01,
        0xac_10_00_01,
        0x80_00_00_00,
        0x00_ff_00_ff,
    ];
    for long in longs {
        let ip = long_to_ip(long);
        assert_eq!(ip_to_binary_string(&ip).unwrap(), format!("{:032b}", long));
        assert_eq!(ip_to_long(&ip).unwrap(), long);
    }
}
