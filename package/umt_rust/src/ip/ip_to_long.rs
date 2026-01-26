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
