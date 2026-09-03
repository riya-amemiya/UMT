use crate::ip::ip_to_long::ip_to_long;

/// Converts an IPv4 address to its binary string representation
///
/// # Arguments
/// * `ip` - IPv4 address (e.g., "192.168.1.1")
///
/// # Returns
/// * `Ok(String)` - Binary string representation (32 bits)
/// * `Err` - If IP address is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::ip_to_binary_string;
/// assert_eq!(ip_to_binary_string("192.168.1.1").unwrap(), "11000000101010000000000100000001");
/// ```
pub fn ip_to_binary_string(ip: &str) -> Result<String, String> {
    Ok(format!("{:032b}", ip_to_long(ip)?))
}
