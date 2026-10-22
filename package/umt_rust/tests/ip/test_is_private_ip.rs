use umt_rust::ip::{is_in_range, is_private_ip};

// Test private IP addresses
#[test]
fn test_is_private_ip_class_a_private_network_address() {
    assert!(is_private_ip("10.0.0.0").unwrap());
}

#[test]
fn test_is_private_ip_class_a_private_start_range() {
    assert!(is_private_ip("10.0.0.1").unwrap());
}

#[test]
fn test_is_private_ip_class_a_private_end_range() {
    assert!(is_private_ip("10.255.255.255").unwrap());
}

#[test]
fn test_is_private_ip_class_b_private_network_address() {
    assert!(is_private_ip("172.16.0.0").unwrap());
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
fn test_is_private_ip_class_c_private_network_address() {
    assert!(is_private_ip("192.168.0.0").unwrap());
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

#[test]
fn test_is_private_ip_unspecified_address() {
    assert!(!is_private_ip("0.0.0.0").unwrap());
}

#[test]
fn test_is_private_ip_broadcast_address() {
    assert!(!is_private_ip("255.255.255.255").unwrap());
}

#[test]
fn test_is_private_ip_matches_is_in_range_rfc1918() {
    let samples = [
        "10.0.0.0",
        "10.0.0.1",
        "10.255.255.255",
        "172.16.0.0",
        "172.16.0.1",
        "172.31.255.255",
        "192.168.0.0",
        "192.168.1.1",
        "192.168.255.255",
        "9.255.255.255",
        "11.0.0.0",
        "172.15.255.255",
        "172.32.0.0",
        "192.167.255.255",
        "192.169.0.0",
        "8.8.8.8",
        "1.1.1.1",
        "169.254.0.1",
        "127.0.0.1",
        "0.0.0.0",
        "255.255.255.255",
    ];
    for ip in samples {
        let expected = is_in_range(ip, "10.0.0.0", 8).unwrap()
            || is_in_range(ip, "172.16.0.0", 12).unwrap()
            || is_in_range(ip, "192.168.0.0", 16).unwrap();
        assert_eq!(is_private_ip(ip).unwrap(), expected, "{ip}");
    }
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
