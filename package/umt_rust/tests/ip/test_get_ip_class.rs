use umt_rust::ip::get_ip_class;

// Test valid IP addresses - Class A
#[test]
fn test_get_ip_class_class_a_start() {
    assert_eq!(get_ip_class("1.0.0.0"), "A"); // Start of class A
}

#[test]
fn test_get_ip_class_class_a_end() {
    assert_eq!(get_ip_class("126.0.0.1"), "A"); // End of class A
}

// Test valid IP addresses - Class B
#[test]
fn test_get_ip_class_class_b_start() {
    assert_eq!(get_ip_class("128.0.0.1"), "B"); // Start of class B
}

#[test]
fn test_get_ip_class_class_b_end() {
    assert_eq!(get_ip_class("191.255.0.1"), "B"); // End of class B
}

// Test valid IP addresses - Class C
#[test]
fn test_get_ip_class_class_c_start() {
    assert_eq!(get_ip_class("192.0.0.1"), "C"); // Start of class C
}

#[test]
fn test_get_ip_class_class_c_end() {
    assert_eq!(get_ip_class("223.255.0.1"), "C"); // End of class C
}

// Test valid IP addresses - Class D
#[test]
fn test_get_ip_class_class_d_start() {
    assert_eq!(get_ip_class("224.0.0.1"), "D"); // Start of class D
}

#[test]
fn test_get_ip_class_class_d_end() {
    assert_eq!(get_ip_class("239.0.0.1"), "D"); // End of class D
}

// Test valid IP addresses - Class E
#[test]
fn test_get_ip_class_class_e_start() {
    assert_eq!(get_ip_class("240.0.0.1"), "E"); // Start of class E
}

#[test]
fn test_get_ip_class_class_e_end() {
    assert_eq!(get_ip_class("255.255.255.255"), "E"); // End of class E
}

// Test invalid IP addresses - should return empty string
#[test]
fn test_get_ip_class_empty_string() {
    assert_eq!(get_ip_class(""), ""); // empty string
}

#[test]
fn test_get_ip_class_reserved_address() {
    assert_eq!(get_ip_class("0.0.0.0"), ""); // reserved address
}

#[test]
fn test_get_ip_class_first_octet_too_large() {
    assert_eq!(get_ip_class("256.0.0.1"), ""); // first octet > 255
}

#[test]
fn test_get_ip_class_incomplete_ip() {
    assert_eq!(get_ip_class("192.168"), ""); // incomplete IP
}

#[test]
fn test_get_ip_class_too_many_octets() {
    assert_eq!(get_ip_class("192.168.1.1.1"), ""); // too many octets
}

#[test]
fn test_get_ip_class_non_numeric_octets() {
    assert_eq!(get_ip_class("a.b.c.d"), ""); // non-numeric octets
}

#[test]
fn test_get_ip_class_too_few_octets() {
    assert_eq!(get_ip_class("192.168.1"), ""); // too few octets
}

#[test]
fn test_get_ip_class_negative_octet() {
    assert_eq!(get_ip_class("-1.0.0.0"), ""); // negative octet
}

#[test]
fn test_get_ip_class_too_many_segments() {
    assert_eq!(get_ip_class("1.2.3.4.5"), ""); // too many segments
}
