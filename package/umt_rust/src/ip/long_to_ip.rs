/// Error type for IP conversion
#[derive(Debug, Clone, PartialEq)]
pub struct LongToIpError {
    pub message: String,
}

impl std::fmt::Display for LongToIpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LongToIpError {}

/// Converts a 32-bit number to an IPv4 address string.
///
/// # Arguments
///
/// * `long` - A 32-bit unsigned integer representing an IP address
///
/// # Returns
///
/// A `Result` containing the IPv4 address string (e.g., "192.168.1.1"),
/// or a `LongToIpError` if the input is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_long_to_ip;
///
/// assert_eq!(umt_long_to_ip(0xC0A80001).unwrap(), "192.168.0.1");
/// assert_eq!(umt_long_to_ip(0x7F000001).unwrap(), "127.0.0.1");
/// ```
#[inline]
pub fn umt_long_to_ip(long: u32) -> Result<String, LongToIpError> {
    // Extract each octet from the 32-bit integer
    let octet1 = (long >> 24) & 0xFF;
    let octet2 = (long >> 16) & 0xFF;
    let octet3 = (long >> 8) & 0xFF;
    let octet4 = long & 0xFF;

    Ok(format!("{}.{}.{}.{}", octet1, octet2, octet3, octet4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_long_values() {
        assert_eq!(umt_long_to_ip(0xC0A80001).unwrap(), "192.168.0.1"); // Common private network
        assert_eq!(umt_long_to_ip(0x80000001).unwrap(), "128.0.0.1"); // Class B start
        assert_eq!(umt_long_to_ip(0x0A000001).unwrap(), "10.0.0.1"); // Class A private
        assert_eq!(umt_long_to_ip(0xAC100001).unwrap(), "172.16.0.1"); // Class B private
        assert_eq!(umt_long_to_ip(0xFFFFFFFF).unwrap(), "255.255.255.255"); // Maximum value
        assert_eq!(umt_long_to_ip(0x00000000).unwrap(), "0.0.0.0"); // Minimum value
        assert_eq!(umt_long_to_ip(0x7F000001).unwrap(), "127.0.0.1"); // Localhost
        assert_eq!(umt_long_to_ip(0x01020304).unwrap(), "1.2.3.4"); // Simple incremental
    }
}
