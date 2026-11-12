use crate::ip::ip_to_long::ip_to_long;

/// Checks whether a string is a valid IPv4 address in dotted-quad notation
///
/// Accepts exactly four ASCII decimal octets in the range 0-255 without
/// leading zeros, the same input that `ip_to_long` accepts.
///
/// # Arguments
/// * `ip` - String to check (e.g., "192.168.1.1")
///
/// # Returns
/// `true` if the string is a valid IPv4 address, `false` otherwise
///
/// # Examples
/// ```
/// use umt_rust::ip::is_valid_ip;
/// assert!(is_valid_ip("192.168.1.1"));
/// assert!(!is_valid_ip("192.168.01.1"));
/// assert!(!is_valid_ip("256.0.0.1"));
/// ```
pub fn is_valid_ip(ip: &str) -> bool {
    ip_to_long(ip).is_ok()
}
