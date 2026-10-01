use crate::ip::cidr_to_long::cidr_to_long;
use crate::ip::ip_to_long::ip_to_long;
use crate::ip::subnet_mask_to_cidr::subnet_mask_to_cidr;

/// Calculates the network address from an IP address and subnet mask
///
/// # Arguments
/// * `ip` - IPv4 address (e.g., "192.168.1.1")
/// * `subnet_mask` - Subnet mask (e.g., "255.255.255.0")
///
/// # Returns
/// * `Ok(u32)` - Network address as a 32-bit unsigned integer
/// * `Err` - If IP address or subnet mask is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::get_network_address;
/// let result = get_network_address("192.168.1.100", "255.255.255.0").unwrap();
/// assert_eq!(result, 0xC0A80100); // 192.168.1.0
/// ```
pub fn get_network_address(ip: &str, subnet_mask: &str) -> Result<u32, String> {
    if ip.is_empty() {
        return Err("IP address is required".to_string());
    }
    if subnet_mask.is_empty() {
        return Err("Subnet mask is required".to_string());
    }

    let ip_long = ip_to_long(ip).map_err(|_| "Invalid IP address or subnet mask".to_string())?;
    let cidr = subnet_mask_to_cidr(subnet_mask)
        .map_err(|_| "Invalid IP address or subnet mask".to_string())?;
    let mask = cidr_to_long(cidr).map_err(|_| "Invalid IP address or subnet mask".to_string())?;

    Ok(ip_long & mask)
}
