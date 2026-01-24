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
        ("10.0.0.0", 8),      // Class A private network
        ("172.16.0.0", 12),   // Class B private network
        ("192.168.0.0", 16),  // Class C private network
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_ip_private() {
        // Class A private (10.0.0.0/8)
        assert!(is_private_ip("10.0.0.1").unwrap());
        assert!(is_private_ip("10.255.255.255").unwrap());

        // Class B private (172.16.0.0/12)
        assert!(is_private_ip("172.16.0.1").unwrap());
        assert!(is_private_ip("172.31.255.255").unwrap());

        // Class C private (192.168.0.0/16)
        assert!(is_private_ip("192.168.0.1").unwrap());
        assert!(is_private_ip("192.168.255.255").unwrap());
    }

    #[test]
    fn test_is_private_ip_public() {
        assert!(!is_private_ip("8.8.8.8").unwrap());
        assert!(!is_private_ip("1.1.1.1").unwrap());
        assert!(!is_private_ip("172.32.0.1").unwrap()); // Just outside 172.16.0.0/12
        assert!(!is_private_ip("11.0.0.1").unwrap()); // Just outside 10.0.0.0/8
    }

    #[test]
    fn test_is_private_ip_invalid() {
        assert!(is_private_ip("").is_err());
        assert!(is_private_ip("invalid").is_err());
    }
}
