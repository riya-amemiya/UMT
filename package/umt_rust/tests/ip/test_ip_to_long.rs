use umt_rust::ip::ip_to_long;

// Test valid IP addresses
#[test]
fn test_ip_to_long_common_private_network() {
    assert_eq!(ip_to_long("192.168.0.1").unwrap(), 0xc0_a8_00_01);
}

#[test]
fn test_ip_to_long_class_b_start() {
    assert_eq!(ip_to_long("128.0.0.1").unwrap(), 0x80_00_00_01);
}

#[test]
fn test_ip_to_long_class_a_private() {
    assert_eq!(ip_to_long("10.0.0.1").unwrap(), 0x0a_00_00_01);
}

#[test]
fn test_ip_to_long_class_b_private() {
    assert_eq!(ip_to_long("172.16.0.1").unwrap(), 0xac_10_00_01);
}

#[test]
fn test_ip_to_long_maximum_value() {
    assert_eq!(ip_to_long("255.255.255.255").unwrap(), 0xff_ff_ff_ff);
}

#[test]
fn test_ip_to_long_minimum_value() {
    assert_eq!(ip_to_long("0.0.0.0").unwrap(), 0x00_00_00_00);
}

#[test]
fn test_ip_to_long_localhost() {
    assert_eq!(ip_to_long("127.0.0.1").unwrap(), 0x7f_00_00_01);
}

#[test]
fn test_ip_to_long_simple_incremental() {
    assert_eq!(ip_to_long("1.2.3.4").unwrap(), 0x01_02_03_04);
}

// Test invalid IP addresses
#[test]
fn test_ip_to_long_empty_string() {
    let result = ip_to_long("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "IP address is required");
}

#[test]
fn test_ip_to_long_incomplete_ip() {
    let result = ip_to_long("192.168");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_octet_too_large() {
    let result = ip_to_long("256.1.2.3");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_non_numeric_octets() {
    let result = ip_to_long("a.b.c.d");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_negative_octet() {
    let result = ip_to_long("-1.0.0.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_too_many_octets() {
    let result = ip_to_long("192.168.1.1.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_double_dot() {
    let result = ip_to_long("192.168..1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_leading_zero_third_octet() {
    let result = ip_to_long("192.168.01.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_leading_zero_fourth_octet() {
    let result = ip_to_long("192.168.1.01");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_all_leading_zeros() {
    let result = ip_to_long("010.020.030.040");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_trailing_dot() {
    let result = ip_to_long("192.168.1.");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_leading_dot() {
    let result = ip_to_long(".192.168.1");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid IP address format");
}

#[test]
fn test_ip_to_long_packs_octets_as_big_endian_u32() {
    assert_eq!(
        ip_to_long("192.168.1.1").unwrap(),
        (192u32 << 24) | (168 << 16) | (1 << 8) | 1
    );
    assert_eq!(
        ip_to_long("10.20.30.40").unwrap(),
        (10u32 << 24) | (20 << 16) | (30 << 8) | 40
    );
}
