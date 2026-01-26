use umt_rust::ip::cidr_to_subnet_mask;

// Test valid CIDR values
#[test]
fn test_cidr_to_subnet_mask_cidr_32() {
    assert_eq!(cidr_to_subnet_mask(32).unwrap(), "255.255.255.255"); // Full mask
}

#[test]
fn test_cidr_to_subnet_mask_cidr_24() {
    assert_eq!(cidr_to_subnet_mask(24).unwrap(), "255.255.255.0"); // Common class C
}

#[test]
fn test_cidr_to_subnet_mask_cidr_16() {
    assert_eq!(cidr_to_subnet_mask(16).unwrap(), "255.255.0.0"); // Common class B
}

#[test]
fn test_cidr_to_subnet_mask_cidr_8() {
    assert_eq!(cidr_to_subnet_mask(8).unwrap(), "255.0.0.0"); // Common class A
}

#[test]
fn test_cidr_to_subnet_mask_cidr_0() {
    assert_eq!(cidr_to_subnet_mask(0).unwrap(), "0.0.0.0"); // No mask
}

#[test]
fn test_cidr_to_subnet_mask_cidr_1() {
    assert_eq!(cidr_to_subnet_mask(1).unwrap(), "128.0.0.0"); // First bit
}

#[test]
fn test_cidr_to_subnet_mask_cidr_31() {
    assert_eq!(cidr_to_subnet_mask(31).unwrap(), "255.255.255.254"); // Second to last
}

#[test]
fn test_cidr_to_subnet_mask_cidr_28() {
    assert_eq!(cidr_to_subnet_mask(28).unwrap(), "255.255.255.240"); // Common subnet
}

#[test]
fn test_cidr_to_subnet_mask_cidr_20() {
    assert_eq!(cidr_to_subnet_mask(20).unwrap(), "255.255.240.0"); // Less common subnet
}

// Test invalid CIDR values
#[test]
fn test_cidr_to_subnet_mask_invalid_cidr_33() {
    let result = cidr_to_subnet_mask(33);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "CIDR must be an integer between 0 and 32"
    );
}

#[test]
fn test_cidr_to_subnet_mask_invalid_cidr_255() {
    // Maximum u8 value that's invalid
    let result = cidr_to_subnet_mask(255);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "CIDR must be an integer between 0 and 32"
    );
}

use umt_rust::ip::*;

#[test]
fn test_cidr_to_subnet_mask() {
    assert_eq!(cidr_to_subnet_mask(0).unwrap(), "0.0.0.0");
    assert_eq!(cidr_to_subnet_mask(8).unwrap(), "255.0.0.0");
    assert_eq!(cidr_to_subnet_mask(16).unwrap(), "255.255.0.0");
    assert_eq!(cidr_to_subnet_mask(24).unwrap(), "255.255.255.0");
    assert_eq!(cidr_to_subnet_mask(32).unwrap(), "255.255.255.255");
}

#[test]
fn test_cidr_to_subnet_mask_invalid() {
    assert!(cidr_to_subnet_mask(33).is_err());
}
