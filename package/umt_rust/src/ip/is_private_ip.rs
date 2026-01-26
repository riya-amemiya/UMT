use crate::ip::is_in_range::is_in_range;

/// Checks if an IP address is within private IP ranges
///
/// # Arguments
/// * `ip` - IP address to check (e.g., "192.168.1.1")
///
/// # Returns
/// * `Ok(bool)` - True if the IP is private, false otherwise
/// * `Err` - If IP address is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::is_private_ip;
/// assert!(is_private_ip("192.168.1.1").unwrap());
/// assert!(is_private_ip("10.0.0.1").unwrap());
/// assert!(!is_private_ip("8.8.8.8").unwrap());
/// ```
pub fn is_private_ip(ip: &str) -> Result<bool, String> {
    if ip.is_empty() {
        return Err("IP address is required".to_string());
    }

    // Define private IP ranges with their CIDR notations
    let private_ranges = [
        ("10.0.0.0", 8),     // Class A private network
        ("172.16.0.0", 12),  // Class B private network
        ("192.168.0.0", 16), // Class C private network
    ];

    for (network, cidr) in private_ranges {
        match is_in_range(ip, network, cidr) {
            Ok(true) => return Ok(true),
            Ok(false) => continue,
            Err(e) => return Err(format!("Invalid IP address: {}", e)),
        }
    }

    Ok(false)
}
