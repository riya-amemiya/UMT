use std::sync::LazyLock;

use crate::ip::cidr_to_long::cidr_to_long;
use crate::ip::ip_to_long::ip_to_long;

static PRIVATE_RANGES: LazyLock<[(u32, u32); 3]> = LazyLock::new(|| {
    [
        (ip_to_long("10.0.0.0").unwrap(), cidr_to_long(8).unwrap()),
        (ip_to_long("172.16.0.0").unwrap(), cidr_to_long(12).unwrap()),
        (
            ip_to_long("192.168.0.0").unwrap(),
            cidr_to_long(16).unwrap(),
        ),
    ]
});

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

    let remote_long = ip_to_long(ip).map_err(|e| format!("Invalid IP address: {}", e))?;

    Ok(PRIVATE_RANGES
        .iter()
        .any(|&(network, mask)| (remote_long & mask) == (network & mask)))
}
