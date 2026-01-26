use umt_rust::ip::subnet_mask_to_cidr;

// Test valid subnet masks
#[test]
fn test_subnet_mask_to_cidr_all_bits_set() {
    assert_eq!(subnet_mask_to_cidr("255.255.255.255").unwrap(), 32);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_24() {
    assert_eq!(subnet_mask_to_cidr("255.255.255.0").unwrap(), 24);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_16() {
    assert_eq!(subnet_mask_to_cidr("255.255.0.0").unwrap(), 16);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_8() {
    assert_eq!(subnet_mask_to_cidr("255.0.0.0").unwrap(), 8);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_23() {
    assert_eq!(subnet_mask_to_cidr("255.255.254.0").unwrap(), 23);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_30() {
    assert_eq!(subnet_mask_to_cidr("255.255.255.252").unwrap(), 30);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_29() {
    assert_eq!(subnet_mask_to_cidr("255.255.255.248").unwrap(), 29);
}

#[test]
fn test_subnet_mask_to_cidr_cidr_20() {
    assert_eq!(subnet_mask_to_cidr("255.255.240.0").unwrap(), 20);
}

#[test]
fn test_subnet_mask_to_cidr_no_bits_set() {
    assert_eq!(subnet_mask_to_cidr("0.0.0.0").unwrap(), 0);
}

// Test invalid subnet masks
#[test]
fn test_subnet_mask_to_cidr_empty_string() {
    let result = subnet_mask_to_cidr("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Subnet mask is required");
}

#[test]
fn test_subnet_mask_to_cidr_incomplete_mask() {
    let result = subnet_mask_to_cidr("192.168");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_first_octet_too_large() {
    let result = subnet_mask_to_cidr("256.255.255.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_last_octet_too_large() {
    let result = subnet_mask_to_cidr("255.255.255.256");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_negative_octet() {
    let result = subnet_mask_to_cidr("255.-1.255.0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_non_numeric_octet() {
    let result = subnet_mask_to_cidr("255.255.255.abc");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_all_non_numeric() {
    let result = subnet_mask_to_cidr("a.b.c.d");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid subnet mask format");
}

#[test]
fn test_subnet_mask_to_cidr_non_continuous_mask_ending_with_1() {
    let result = subnet_mask_to_cidr("255.255.255.1");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid subnet mask: must be consecutive 1s followed by 0s"
    );
}

#[test]
fn test_subnet_mask_to_cidr_non_continuous_mask_gap() {
    let result = subnet_mask_to_cidr("255.0.255.0");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid subnet mask: must be consecutive 1s followed by 0s"
    );
}

#[test]
fn test_subnet_mask_to_cidr_invalid_bit_pattern() {
    let result = subnet_mask_to_cidr("254.255.255.0");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid subnet mask: must be consecutive 1s followed by 0s"
    );
}

use umt_rust::ip::*;

#[test]
fn test_subnet_mask_to_cidr_invalid() {
    assert!(subnet_mask_to_cidr("").is_err());
    assert!(subnet_mask_to_cidr("255.255.255").is_err());
    assert!(subnet_mask_to_cidr("255.0.255.0").is_err()); // Non-consecutive
    assert!(subnet_mask_to_cidr("invalid").is_err());
}

#[test]
fn test_subnet_mask_to_cidr_valid() {
    assert_eq!(subnet_mask_to_cidr("0.0.0.0").unwrap(), 0);
    assert_eq!(subnet_mask_to_cidr("255.0.0.0").unwrap(), 8);
    assert_eq!(subnet_mask_to_cidr("255.255.0.0").unwrap(), 16);
    assert_eq!(subnet_mask_to_cidr("255.255.255.0").unwrap(), 24);
    assert_eq!(subnet_mask_to_cidr("255.255.255.255").unwrap(), 32);
    assert_eq!(subnet_mask_to_cidr("255.255.255.128").unwrap(), 25);
}
