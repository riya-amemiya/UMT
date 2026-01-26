use umt_rust::ip::cidr_to_long;

// Test valid CIDR values
#[test]
fn test_cidr_to_long_cidr_32() {
    assert_eq!(cidr_to_long(32).unwrap(), 0xff_ff_ff_ff); // 255.255.255.255
}

#[test]
fn test_cidr_to_long_cidr_24() {
    assert_eq!(cidr_to_long(24).unwrap(), 0xff_ff_ff_00); // 255.255.255.0
}

#[test]
fn test_cidr_to_long_cidr_16() {
    assert_eq!(cidr_to_long(16).unwrap(), 0xff_ff_00_00); // 255.255.0.0
}

#[test]
fn test_cidr_to_long_cidr_8() {
    assert_eq!(cidr_to_long(8).unwrap(), 0xff_00_00_00); // 255.0.0.0
}

#[test]
fn test_cidr_to_long_cidr_0() {
    assert_eq!(cidr_to_long(0).unwrap(), 0x00_00_00_00); // 0.0.0.0
}

#[test]
fn test_cidr_to_long_cidr_1() {
    assert_eq!(cidr_to_long(1).unwrap(), 0x80_00_00_00); // 128.0.0.0
}

#[test]
fn test_cidr_to_long_cidr_31() {
    assert_eq!(cidr_to_long(31).unwrap(), 0xff_ff_ff_fe); // 255.255.255.254
}

// Test invalid CIDR values
#[test]
fn test_cidr_to_long_invalid_cidr_33() {
    let result = cidr_to_long(33);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "CIDR must be an integer between 0 and 32"
    );
}

#[test]
fn test_cidr_to_long_invalid_cidr_255() {
    // Maximum u8 value that's invalid
    let result = cidr_to_long(255);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "CIDR must be an integer between 0 and 32"
    );
}
