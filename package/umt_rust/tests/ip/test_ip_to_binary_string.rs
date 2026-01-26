use umt_rust::ip::ip_to_binary_string;

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

use umt_rust::ip::*;

#[test]
fn test_ip_to_binary_string_invalid() {
    assert!(ip_to_binary_string("").is_err());
    assert!(ip_to_binary_string("192.168.1").is_err());
    assert!(ip_to_binary_string("192.168.1.1.1").is_err());
    assert!(ip_to_binary_string("192.168.01.1").is_err());
    assert!(ip_to_binary_string("192.168.1.256").is_err());
    assert!(ip_to_binary_string("abc.def.ghi.jkl").is_err());
}

#[test]
fn test_ip_to_binary_string_valid() {
    assert_eq!(
        ip_to_binary_string("192.168.1.1").unwrap(),
        "11000000101010000000000100000001"
    );
    assert_eq!(
        ip_to_binary_string("0.0.0.0").unwrap(),
        "00000000000000000000000000000000"
    );
    assert_eq!(
        ip_to_binary_string("255.255.255.255").unwrap(),
        "11111111111111111111111111111111"
    );
}
