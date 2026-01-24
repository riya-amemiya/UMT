/// Gets the IP address class (A, B, C, D, or E).
///
/// # Arguments
///
/// * `ip` - IPv4 address
///
/// # Returns
///
/// The IP class as a string ('A', 'B', 'C', 'D', 'E', or empty string for invalid IP).
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_get_ip_class;
///
/// assert_eq!(umt_get_ip_class("10.0.0.1"), "A");
/// assert_eq!(umt_get_ip_class("172.16.0.1"), "B");
/// assert_eq!(umt_get_ip_class("192.168.1.1"), "C");
/// ```
#[inline]
pub fn umt_get_ip_class(ip: &str) -> String {
    if ip.is_empty() {
        return String::new();
    }

    // Validate IP format
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return String::new();
    }

    // Parse the first octet
    let first_octet: i32 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return String::new(),
    };

    if first_octet < 0 || first_octet > 255 {
        return String::new();
    }

    // Check each class range
    match first_octet {
        0 => String::new(),       // Reserved
        1..=127 => "A".to_string(), // Class A: 1-127
        128..=191 => "B".to_string(), // Class B: 128-191
        192..=223 => "C".to_string(), // Class C: 192-223
        224..=239 => "D".to_string(), // Class D: 224-239
        240..=255 => "E".to_string(), // Class E: 240-255
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_addresses() {
        // Class A
        assert_eq!(umt_get_ip_class("1.0.0.0"), "A"); // Start of class A
        assert_eq!(umt_get_ip_class("126.0.0.1"), "A"); // End of class A

        // Class B
        assert_eq!(umt_get_ip_class("128.0.0.1"), "B"); // Start of class B
        assert_eq!(umt_get_ip_class("191.255.0.1"), "B"); // End of class B

        // Class C
        assert_eq!(umt_get_ip_class("192.0.0.1"), "C"); // Start of class C
        assert_eq!(umt_get_ip_class("223.255.0.1"), "C"); // End of class C

        // Class D
        assert_eq!(umt_get_ip_class("224.0.0.1"), "D"); // Start of class D
        assert_eq!(umt_get_ip_class("239.0.0.1"), "D"); // End of class D

        // Class E
        assert_eq!(umt_get_ip_class("240.0.0.1"), "E"); // Start of class E
        assert_eq!(umt_get_ip_class("255.255.255.255"), "E"); // End of class E
    }

    #[test]
    fn test_invalid_ip_addresses() {
        assert_eq!(umt_get_ip_class(""), ""); // empty string
        assert_eq!(umt_get_ip_class("0.0.0.0"), ""); // reserved address
        assert_eq!(umt_get_ip_class("256.0.0.1"), ""); // first octet > 255
        assert_eq!(umt_get_ip_class("192.168"), ""); // incomplete IP
        assert_eq!(umt_get_ip_class("192.168.1.1.1"), ""); // too many octets
        assert_eq!(umt_get_ip_class("a.b.c.d"), ""); // non-numeric octets
        assert_eq!(umt_get_ip_class("192.168.1"), ""); // too few octets
        assert_eq!(umt_get_ip_class("-1.0.0.0"), ""); // negative octet
        assert_eq!(umt_get_ip_class("1.2.3.4.5"), ""); // too many segments
    }
}
