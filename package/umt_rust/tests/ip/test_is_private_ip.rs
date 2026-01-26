use umt_rust::ip::is_private_ip;

// Test private IP addresses
#[test]
fn test_is_private_ip_class_a_private_start_range() {
    assert!(is_private_ip("10.0.0.1").unwrap());
}

#[test]
fn test_is_private_ip_class_a_private_end_range() {
    assert!(is_private_ip("10.255.255.255").unwrap());
}

#[test]
fn test_is_private_ip_class_b_private_start_range() {
    assert!(is_private_ip("172.16.0.1").unwrap());
}

#[test]
fn test_is_private_ip_class_b_private_end_range() {
    assert!(is_private_ip("172.31.255.255").unwrap());
}

#[test]
fn test_is_private_ip_class_c_private_start_range() {
    assert!(is_private_ip("192.168.0.1").unwrap());
}

#[test]
fn test_is_private_ip_class_c_private_end_range() {
    assert!(is_private_ip("192.168.255.255").unwrap());
}

// Test non-private IP addresses
#[test]
fn test_is_private_ip_just_before_class_a_private() {
    assert!(!is_private_ip("9.255.255.255").unwrap());
}

#[test]
fn test_is_private_ip_just_after_class_a_private() {
    assert!(!is_private_ip("11.0.0.0").unwrap());
}

#[test]
fn test_is_private_ip_just_before_class_b_private() {
    assert!(!is_private_ip("172.15.255.255").unwrap());
}

#[test]
fn test_is_private_ip_just_after_class_b_private() {
    assert!(!is_private_ip("172.32.0.0").unwrap());
}

#[test]
fn test_is_private_ip_just_before_class_c_private() {
    assert!(!is_private_ip("192.167.255.255").unwrap());
}

#[test]
fn test_is_private_ip_just_after_class_c_private() {
    assert!(!is_private_ip("192.169.0.0").unwrap());
}

#[test]
fn test_is_private_ip_google_dns() {
    assert!(!is_private_ip("8.8.8.8").unwrap());
}

#[test]
fn test_is_private_ip_cloudflare_dns() {
    assert!(!is_private_ip("1.1.1.1").unwrap());
}

#[test]
fn test_is_private_ip_link_local_address() {
    assert!(!is_private_ip("169.254.0.1").unwrap());
}

#[test]
fn test_is_private_ip_localhost() {
    assert!(!is_private_ip("127.0.0.1").unwrap());
}

// Test invalid inputs
#[test]
fn test_is_private_ip_empty_string() {
    let result = is_private_ip("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "IP address is required");
}

#[test]
fn test_is_private_ip_octet_out_of_range() {
    let result = is_private_ip("256.256.256.256");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address"));
}

#[test]
fn test_is_private_ip_incomplete_ip() {
    let result = is_private_ip("192.168");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address"));
}

#[test]
fn test_is_private_ip_non_numeric_octets() {
    let result = is_private_ip("a.b.c.d");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address"));
}

#[test]
fn test_is_private_ip_too_many_octets() {
    let result = is_private_ip("192.168.1.1.1");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address"));
}

#[test]
fn test_is_private_ip_negative_octet() {
    let result = is_private_ip("-1.0.0.0");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address"));
}

use umt_rust::ip::*;

#[test]
fn test_is_private_ip_invalid() {
    assert!(is_private_ip("").is_err());
    assert!(is_private_ip("invalid").is_err());
}

#[test]
fn test_is_private_ip_private() {
    // Class A private (10.0.0.0/8)
    assert!(is_private_ip("10.0.0.1").unwrap());
    assert!(is_private_ip("10.255.255.255").unwrap());

    // Class B private (172.16.0.0/12)
    assert!(is_private_ip("172.16.0.1").unwrap());
    assert!(is_private_ip("172.31.255.255").unwrap());

    // Class C private (192.168.0.0/16)
    assert!(is_private_ip("192.168.0.1").unwrap());
    assert!(is_private_ip("192.168.255.255").unwrap());
}

#[test]
fn test_is_private_ip_public() {
    assert!(!is_private_ip("8.8.8.8").unwrap());
    assert!(!is_private_ip("1.1.1.1").unwrap());
    assert!(!is_private_ip("172.32.0.1").unwrap()); // Just outside 172.16.0.0/12
    assert!(!is_private_ip("11.0.0.1").unwrap()); // Just outside 10.0.0.0/8
}
