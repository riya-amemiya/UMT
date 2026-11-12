use crate::ip::cidr_to_long::cidr_to_long;
use crate::ip::ip_to_long::ip_to_long;
use crate::ip::long_to_ip::long_to_ip;

/// Network address and prefix length parsed from CIDR notation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCidr {
    /// Network address with the host bits cleared (e.g., "192.168.0.0")
    pub network_address: String,
    /// Prefix length (0-32)
    pub prefix_length: u8,
}

/// Parses CIDR notation into its network address and prefix length
///
/// The host bits of the address are cleared, so "192.168.1.130/25" gives
/// the network address "192.168.1.128".
///
/// # Arguments
/// * `cidr` - CIDR notation (e.g., "192.168.0.0/24")
///
/// # Returns
/// * `Ok(ParsedCidr)` - Network address and prefix length
/// * `Err` - If the address or the prefix length is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::parse_cidr;
/// let parsed = parse_cidr("192.168.1.130/25").unwrap();
/// assert_eq!(parsed.network_address, "192.168.1.128");
/// assert_eq!(parsed.prefix_length, 25);
/// assert!(parse_cidr("192.168.0.0/33").is_err());
/// ```
pub fn parse_cidr(cidr: &str) -> Result<ParsedCidr, String> {
    let (address, prefix) = cidr
        .split_once('/')
        .ok_or_else(|| "Invalid CIDR notation".to_string())?;

    if prefix.is_empty()
        || prefix.len() > 2
        || !prefix.bytes().all(|b| b.is_ascii_digit())
        || (prefix.len() > 1 && prefix.starts_with('0'))
    {
        return Err("Invalid CIDR notation".to_string());
    }

    let prefix_length: u8 = prefix
        .parse()
        .map_err(|_| "Invalid CIDR notation".to_string())?;
    let mask = cidr_to_long(prefix_length).map_err(|_| "Invalid CIDR notation".to_string())?;

    Ok(ParsedCidr {
        network_address: long_to_ip(ip_to_long(address)? & mask),
        prefix_length,
    })
}
