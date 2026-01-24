use super::ip_to_binary_string::{umt_ip_to_binary_string, IpToBinaryError};

/// Converts an IPv4 address to a 32-bit unsigned integer.
///
/// # Arguments
///
/// * `ip` - IPv4 address to convert (e.g., "192.168.1.1")
///
/// # Returns
///
/// A `Result` containing the 32-bit unsigned integer representation,
/// or an `IpToBinaryError` if the IP address is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_ip_to_long;
///
/// assert_eq!(umt_ip_to_long("192.168.0.1").unwrap(), 0xC0A80001);
/// assert_eq!(umt_ip_to_long("127.0.0.1").unwrap(), 0x7F000001);
/// ```
#[inline]
pub fn umt_ip_to_long(ip: &str) -> Result<u32, IpToBinaryError> {
    let binary_string = umt_ip_to_binary_string(ip)?;
    Ok(u32::from_str_radix(&binary_string, 2).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_addresses() {
        assert_eq!(umt_ip_to_long("192.168.0.1").unwrap(), 0xC0A80001); // Common private network
        assert_eq!(umt_ip_to_long("128.0.0.1").unwrap(), 0x80000001); // Class B start
        assert_eq!(umt_ip_to_long("10.0.0.1").unwrap(), 0x0A000001); // Class A private
        assert_eq!(umt_ip_to_long("172.16.0.1").unwrap(), 0xAC100001); // Class B private
        assert_eq!(umt_ip_to_long("255.255.255.255").unwrap(), 0xFFFFFFFF); // Maximum value
        assert_eq!(umt_ip_to_long("0.0.0.0").unwrap(), 0x00000000); // Minimum value
        assert_eq!(umt_ip_to_long("127.0.0.1").unwrap(), 0x7F000001); // Localhost
        assert_eq!(umt_ip_to_long("1.2.3.4").unwrap(), 0x01020304); // Simple incremental
    }

    #[test]
    fn test_invalid_ip_addresses() {
        assert!(umt_ip_to_long("").is_err());
        assert_eq!(
            umt_ip_to_long("").unwrap_err().message,
            "IP address is required"
        );

        assert!(umt_ip_to_long("192.168").is_err());
        assert!(umt_ip_to_long("256.1.2.3").is_err());
        assert!(umt_ip_to_long("a.b.c.d").is_err());
        assert!(umt_ip_to_long("-1.0.0.0").is_err());
        assert!(umt_ip_to_long("192.168.1.1.1").is_err());
        assert!(umt_ip_to_long("192.168..1").is_err());
    }
}
