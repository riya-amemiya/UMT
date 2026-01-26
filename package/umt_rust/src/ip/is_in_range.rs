use crate::ip::ip_to_long::ip_to_long;

/// Checks if an IP address is within a specified network range
///
/// # Arguments
/// * `remote_ip` - IP address to check (e.g., "192.168.1.1")
/// * `network_ip` - Network IP address (e.g., "192.168.0.0")
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
/// * `Ok(bool)` - True if the IP is in range, false otherwise
/// * `Err` - If any parameter is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::is_in_range;
/// assert!(is_in_range("192.168.1.100", "192.168.0.0", 16).unwrap());
/// assert!(!is_in_range("10.0.0.1", "192.168.0.0", 16).unwrap());
/// ```
pub fn is_in_range(remote_ip: &str, network_ip: &str, cidr: u8) -> Result<bool, String> {
    if remote_ip.is_empty() {
        return Err("Remote IP address is required".to_string());
    }
    if network_ip.is_empty() {
        return Err("Network IP address is required".to_string());
    }
    if cidr > 32 {
        return Err("CIDR must be an integer between 0 and 32".to_string());
    }

    let remote_long =
        ip_to_long(remote_ip).map_err(|e| format!("Invalid IP address format: {}", e))?;
    let network_long =
        ip_to_long(network_ip).map_err(|e| format!("Invalid IP address format: {}", e))?;

    // Special cases
    if cidr == 0 {
        return Ok(true); // All IPs are in range
    }
    if cidr == 32 {
        return Ok(remote_long == network_long); // Exact match required
    }

    // Normal case
    let shift = 32 - cidr;
    let mask = 0xFFFFFFFFu32 << shift;

    Ok((remote_long & mask) == (network_long & mask))
}
