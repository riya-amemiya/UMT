use crate::ip::ip_to_binary_string::ip_to_binary_string;

/// Converts an IPv4 address to a 32-bit number
///
/// # Arguments
/// * `ip` - IPv4 address to convert (e.g., "192.168.1.1")
///
/// # Returns
/// * `Ok(u32)` - 32-bit unsigned integer
/// * `Err` - If IP address is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::ip_to_long;
/// assert_eq!(ip_to_long("192.168.1.1").unwrap(), 3232235777);
/// ```
pub fn ip_to_long(ip: &str) -> Result<u32, String> {
    let binary = ip_to_binary_string(ip)?;
    u32::from_str_radix(&binary, 2).map_err(|e| format!("Failed to parse binary: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_to_long_valid() {
        assert_eq!(ip_to_long("0.0.0.0").unwrap(), 0);
        assert_eq!(ip_to_long("255.255.255.255").unwrap(), 0xFFFFFFFF);
        assert_eq!(ip_to_long("192.168.1.1").unwrap(), 0xC0A80101);
        assert_eq!(ip_to_long("10.0.0.1").unwrap(), 0x0A000001);
    }

    #[test]
    fn test_ip_to_long_invalid() {
        assert!(ip_to_long("").is_err());
        assert!(ip_to_long("invalid").is_err());
    }
}
