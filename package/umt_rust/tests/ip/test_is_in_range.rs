use umt_rust::ip::is_in_range;

// Test valid IP ranges
#[test]
fn test_is_in_range_common_class_c_network_in_range() {
    assert!(is_in_range("192.168.1.2", "192.168.1.0", 24).unwrap());
}

#[test]
fn test_is_in_range_common_class_c_network_outside_range() {
    assert!(!is_in_range("192.168.2.2", "192.168.1.0", 24).unwrap());
}

#[test]
fn test_is_in_range_class_a_network_in_range() {
    assert!(is_in_range("10.0.0.5", "10.0.0.0", 8).unwrap());
}

#[test]
fn test_is_in_range_class_a_network_outside_range() {
    assert!(!is_in_range("11.0.0.5", "10.0.0.0", 8).unwrap());
}

#[test]
fn test_is_in_range_class_b_network_in_range() {
    assert!(is_in_range("172.16.1.1", "172.16.0.0", 16).unwrap());
}

#[test]
fn test_is_in_range_class_b_network_outside_range() {
    assert!(!is_in_range("172.17.1.1", "172.16.0.0", 16).unwrap());
}

#[test]
fn test_is_in_range_single_host_network_exact_match_zero() {
    assert!(is_in_range("192.168.1.0", "192.168.1.0", 32).unwrap());
}

#[test]
fn test_is_in_range_single_host_network_exact_match_one() {
    assert!(is_in_range("192.168.1.1", "192.168.1.1", 32).unwrap());
}

#[test]
fn test_is_in_range_single_host_network_different_ip() {
    assert!(!is_in_range("192.168.1.1", "192.168.1.0", 32).unwrap());
}

#[test]
fn test_is_in_range_small_subnet_in_range() {
    assert!(is_in_range("192.168.1.1", "192.168.1.0", 30).unwrap());
}

#[test]
fn test_is_in_range_small_subnet_outside_range() {
    assert!(!is_in_range("192.168.1.4", "192.168.1.0", 30).unwrap());
}

#[test]
fn test_is_in_range_cidr_0_all_ips_in_range_low() {
    assert!(is_in_range("0.0.0.1", "0.0.0.0", 0).unwrap());
}

#[test]
fn test_is_in_range_cidr_0_all_ips_in_range_high() {
    assert!(is_in_range("255.255.255.255", "0.0.0.0", 0).unwrap());
}

// Additional edge case tests
#[test]
fn test_is_in_range_cidr_0_any_ip() {
    assert!(is_in_range("192.168.1.1", "0.0.0.0", 0).unwrap());
}

#[test]
fn test_is_in_range_cidr_32_exact_match_only() {
    assert!(is_in_range("192.168.1.0", "192.168.1.0", 32).unwrap());
}

#[test]
fn test_is_in_range_cidr_31_two_ips() {
    assert!(is_in_range("192.168.1.1", "192.168.1.0", 31).unwrap());
}

#[test]
fn test_is_in_range_cidr_1_half_of_all_ips() {
    assert!(is_in_range("192.168.1.1", "192.168.1.0", 1).unwrap());
}

// More edge case tests with specific CIDR masks
#[test]
fn test_is_in_range_cidr_24_edge_in_range() {
    assert!(is_in_range("192.168.1.5", "192.168.1.0", 24).unwrap());
}

#[test]
fn test_is_in_range_cidr_24_edge_outside_range() {
    assert!(!is_in_range("192.168.2.0", "192.168.1.0", 24).unwrap());
}

#[test]
fn test_is_in_range_cidr_31_first_ip() {
    assert!(is_in_range("192.168.1.0", "192.168.1.0", 31).unwrap());
}

#[test]
fn test_is_in_range_cidr_31_second_ip() {
    assert!(is_in_range("192.168.1.1", "192.168.1.0", 31).unwrap());
}

#[test]
fn test_is_in_range_cidr_31_outside_range() {
    assert!(!is_in_range("192.168.1.2", "192.168.1.0", 31).unwrap());
}

#[test]
fn test_is_in_range_cidr_30_in_range() {
    assert!(is_in_range("192.168.1.3", "192.168.1.0", 30).unwrap());
}

#[test]
fn test_is_in_range_cidr_30_another_network() {
    assert!(is_in_range("192.168.1.4", "192.168.1.4", 30).unwrap());
}

// Test invalid inputs
#[test]
fn test_is_in_range_empty_remote_ip() {
    let result = is_in_range("", "192.168.1.0", 24);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Remote IP address is required");
}

#[test]
fn test_is_in_range_empty_network_ip() {
    let result = is_in_range("192.168.1.1", "", 24);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Network IP address is required");
}

#[test]
fn test_is_in_range_cidr_too_large() {
    let result = is_in_range("192.168.1.1", "192.168.1.0", 33);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "CIDR must be an integer between 0 and 32"
    );
}

// Test invalid IP formats
#[test]
fn test_is_in_range_invalid_remote_ip() {
    let result = is_in_range("invalid", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_invalid_network_ip() {
    let result = is_in_range("192.168.1.1", "invalid", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_remote_ip_out_of_range() {
    let result = is_in_range("256.256.256.256", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_network_ip_out_of_range() {
    let result = is_in_range("192.168.1.1", "256.256.256.256", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_too_many_octets_remote() {
    let result = is_in_range("192.168.1.1.1", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_negative_octet_remote() {
    let result = is_in_range("192.168.-1.1", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_too_few_octets_remote() {
    let result = is_in_range("192.168.1", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

// Test error message formatting
#[test]
fn test_is_in_range_error_format_invalid_ip() {
    let result = is_in_range("invalid-ip", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_error_format_invalid_network() {
    let result = is_in_range("192.168.1.1", "invalid-network", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

#[test]
fn test_is_in_range_error_format_out_of_range_values() {
    let result = is_in_range("999.999.999.999", "192.168.1.0", 24);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid IP address format"));
}

use umt_rust::ip::*;

#[test]
fn test_is_in_range_invalid() {
    assert!(is_in_range("", "192.168.0.0", 16).is_err());
    assert!(is_in_range("192.168.1.1", "", 16).is_err());
    assert!(is_in_range("192.168.1.1", "192.168.0.0", 33).is_err());
    assert!(is_in_range("invalid", "192.168.0.0", 16).is_err());
}

#[test]
fn test_is_in_range_valid() {
    // Same network
    assert!(is_in_range("192.168.1.100", "192.168.0.0", 16).unwrap());
    assert!(is_in_range("192.168.255.255", "192.168.0.0", 16).unwrap());

    // Different network
    assert!(!is_in_range("10.0.0.1", "192.168.0.0", 16).unwrap());
    assert!(!is_in_range("192.169.0.1", "192.168.0.0", 16).unwrap());

    // Edge cases
    assert!(is_in_range("1.2.3.4", "5.6.7.8", 0).unwrap()); // CIDR 0 matches all
    assert!(is_in_range("192.168.1.1", "192.168.1.1", 32).unwrap()); // Exact match
    assert!(!is_in_range("192.168.1.1", "192.168.1.2", 32).unwrap()); // No match
}
