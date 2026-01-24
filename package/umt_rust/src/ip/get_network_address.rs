use super::cidr_to_long::umt_cidr_to_long;
use super::ip_to_long::umt_ip_to_long;
use super::subnet_mask_to_cidr::umt_subnet_mask_to_cidr;

/// Error type for network address calculation
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkAddressError {
    pub message: String,
}

impl std::fmt::Display for NetworkAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NetworkAddressError {}

/// Calculates the network address from an IP address and subnet mask.
///
/// # Arguments
///
/// * `ip` - IPv4 address (e.g., "192.168.1.1")
/// * `subnet_mask` - Subnet mask (e.g., "255.255.255.0")
///
/// # Returns
///
/// A `Result` containing the network address as a 32-bit unsigned integer,
/// or a `NetworkAddressError` if the IP address or subnet mask is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_get_network_address;
///
/// assert_eq!(umt_get_network_address("192.168.1.1", "255.255.255.0").unwrap(), 0xC0A80100);
/// ```
#[inline]
pub fn umt_get_network_address(ip: &str, subnet_mask: &str) -> Result<u32, NetworkAddressError> {
    if ip.is_empty() {
        return Err(NetworkAddressError {
            message: "IP address is required".to_string(),
        });
    }
    if subnet_mask.is_empty() {
        return Err(NetworkAddressError {
            message: "Subnet mask is required".to_string(),
        });
    }

    // Validate IP format
    let ip_parts: Vec<&str> = ip.split('.').collect();
    if ip_parts.len() != 4 {
        return Err(NetworkAddressError {
            message: "Invalid IP address or subnet mask".to_string(),
        });
    }

    for part in &ip_parts {
        let num: i32 = part.parse().map_err(|_| NetworkAddressError {
            message: "Invalid IP address or subnet mask".to_string(),
        })?;
        if num < 0 || num > 255 {
            return Err(NetworkAddressError {
                message: "Invalid IP address or subnet mask".to_string(),
            });
        }
    }

    // Validate subnet mask format
    let mask_parts: Vec<&str> = subnet_mask.split('.').collect();
    if mask_parts.len() != 4 {
        return Err(NetworkAddressError {
            message: "Invalid IP address or subnet mask".to_string(),
        });
    }

    for part in &mask_parts {
        let num: i32 = part.parse().map_err(|_| NetworkAddressError {
            message: "Invalid IP address or subnet mask".to_string(),
        })?;
        if num < 0 || num > 255 {
            return Err(NetworkAddressError {
                message: "Invalid IP address or subnet mask".to_string(),
            });
        }
    }

    // Calculate network address
    let ip_long = umt_ip_to_long(ip).map_err(|_| NetworkAddressError {
        message: "Invalid IP address or subnet mask".to_string(),
    })?;

    let cidr = umt_subnet_mask_to_cidr(subnet_mask).map_err(|_| NetworkAddressError {
        message: "Invalid IP address or subnet mask".to_string(),
    })?;

    let mask_long = umt_cidr_to_long(cidr).map_err(|_| NetworkAddressError {
        message: "Invalid IP address or subnet mask".to_string(),
    })?;

    Ok(ip_long & mask_long)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_and_subnet_masks() {
        assert_eq!(
            umt_get_network_address("192.168.1.1", "255.255.255.0").unwrap(),
            0xC0A80100
        ); // Common Class C network
        assert_eq!(
            umt_get_network_address("172.16.5.1", "255.255.0.0").unwrap(),
            0xAC100000
        ); // Common Class B network
        assert_eq!(
            umt_get_network_address("10.0.0.15", "255.0.0.0").unwrap(),
            0x0A000000
        ); // Common Class A network
        assert_eq!(
            umt_get_network_address("192.168.1.1", "255.255.254.0").unwrap(),
            0xC0A80000
        ); // Non-standard subnet mask
        assert_eq!(
            umt_get_network_address("255.255.255.255", "255.255.255.0").unwrap(),
            0xFFFFFF00
        ); // All host bits set
        assert_eq!(
            umt_get_network_address("0.0.0.0", "255.255.255.0").unwrap(),
            0x00000000
        ); // All bits zero
        assert_eq!(
            umt_get_network_address("192.168.1.1", "255.255.255.252").unwrap(),
            0xC0A80100
        ); // Small subnet (/30)
        assert_eq!(
            umt_get_network_address("10.10.10.10", "255.255.255.240").unwrap(),
            0x0A0A0A00
        ); // Subnet mask with /28
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(umt_get_network_address("", "255.255.255.0").is_err());
        assert_eq!(
            umt_get_network_address("", "255.255.255.0")
                .unwrap_err()
                .message,
            "IP address is required"
        );

        assert!(umt_get_network_address("192.168.1.1", "").is_err());
        assert_eq!(
            umt_get_network_address("192.168.1.1", "")
                .unwrap_err()
                .message,
            "Subnet mask is required"
        );

        assert!(umt_get_network_address("invalid", "255.255.255.0").is_err());
        assert!(umt_get_network_address("192.168.1.1", "invalid").is_err());
        assert!(umt_get_network_address("256.256.256.256", "255.255.255.0").is_err());
        assert!(umt_get_network_address("192.168.1.1", "256.256.256.256").is_err());
    }

    #[test]
    fn test_internal_function_exceptions() {
        assert!(umt_get_network_address("192.168.1.1", "255.255.128.3").is_err());
        assert_eq!(
            umt_get_network_address("192.168.1.1", "255.255.128.3")
                .unwrap_err()
                .message,
            "Invalid IP address or subnet mask"
        );
    }
}
