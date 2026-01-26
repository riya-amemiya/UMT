use umt_rust::ip::get_network_address;

// Test valid IP addresses and subnet masks
#[test]
fn test_get_network_address_class_c_network() {
    // Common Class C network
    assert_eq!(
        get_network_address("192.168.1.1", "255.255.255.0").unwrap(),
        0xc0_a8_01_00
    );
}

#[test]
fn test_get_network_address_class_b_network() {
    // Common Class B network
    assert_eq!(
        get_network_address("172.16.5.1", "255.255.0.0").unwrap(),
        0xac_10_00_00
    );
}

#[test]
fn test_get_network_address_class_a_network() {
    // Common Class A network
    assert_eq!(
        get_network_address("10.0.0.15", "255.0.0.0").unwrap(),
        0x0a_00_00_00
    );
}

#[test]
fn test_get_network_address_non_standard_subnet_mask() {
    // Non-standard subnet mask
    assert_eq!(
        get_network_address("192.168.1.1", "255.255.254.0").unwrap(),
        0xc0_a8_00_00
    );
}

#[test]
fn test_get_network_address_all_host_bits_set() {
    // All host bits set
    assert_eq!(
        get_network_address("255.255.255.255", "255.255.255.0").unwrap(),
        0xff_ff_ff_00
    );
}

#[test]
fn test_get_network_address_all_bits_zero() {
    // All bits zero
    assert_eq!(
        get_network_address("0.0.0.0", "255.255.255.0").unwrap(),
        0x00_00_00_00
    );
}

#[test]
fn test_get_network_address_small_subnet_cidr_30() {
    // Small subnet (/30)
    assert_eq!(
        get_network_address("192.168.1.1", "255.255.255.252").unwrap(),
        0xc0_a8_01_00
    );
}

#[test]
fn test_get_network_address_subnet_mask_cidr_28() {
    // Subnet mask with /28
    assert_eq!(
        get_network_address("10.10.10.10", "255.255.255.240").unwrap(),
        0x0a_0a_0a_00
    );
}

// Test invalid inputs
#[test]
fn test_get_network_address_empty_ip() {
    let result = get_network_address("", "255.255.255.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "IP address is required");
}

#[test]
fn test_get_network_address_empty_mask() {
    let result = get_network_address("192.168.1.1", "");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Subnet mask is required");
}

#[test]
fn test_get_network_address_invalid_ip_format() {
    let result = get_network_address("invalid", "255.255.255.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address or subnet mask");
}

#[test]
fn test_get_network_address_invalid_mask_format() {
    let result = get_network_address("192.168.1.1", "invalid");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address or subnet mask");
}

#[test]
fn test_get_network_address_ip_out_of_range() {
    let result = get_network_address("256.256.256.256", "255.255.255.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address or subnet mask");
}

#[test]
fn test_get_network_address_mask_out_of_range() {
    let result = get_network_address("192.168.1.1", "256.256.256.256");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address or subnet mask");
}

#[test]
fn test_get_network_address_invalid_subnet_mask_pattern() {
    // Test for exception handling with invalid subnet mask pattern
    let result = get_network_address("192.168.1.1", "255.255.128.3");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address or subnet mask");
}
