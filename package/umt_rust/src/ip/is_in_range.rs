use super::ip_to_long::umt_ip_to_long;

/// Error type for IP range checking
#[derive(Debug, Clone, PartialEq)]
pub struct IsInRangeError {
    pub message: String,
}

impl std::fmt::Display for IsInRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IsInRangeError {}

/// Checks if an IP address is within a specified network range.
///
/// # Arguments
///
/// * `remote_ip` - IP address to check (e.g., "192.168.1.1")
/// * `network_ip` - Network IP address (e.g., "192.168.0.0")
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
///
/// A `Result` containing a boolean indicating whether the IP is in range,
/// or an `IsInRangeError` if any parameter is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_is_in_range;
///
/// assert_eq!(umt_is_in_range("192.168.1.2", "192.168.1.0", 24).unwrap(), true);
/// assert_eq!(umt_is_in_range("192.168.2.2", "192.168.1.0", 24).unwrap(), false);
/// ```
#[inline]
pub fn umt_is_in_range(remote_ip: &str, network_ip: &str, cidr: i32) -> Result<bool, IsInRangeError> {
    if remote_ip.is_empty() {
        return Err(IsInRangeError {
            message: "Remote IP address is required".to_string(),
        });
    }
    if network_ip.is_empty() {
        return Err(IsInRangeError {
            message: "Network IP address is required".to_string(),
        });
    }

    if cidr < 0 || cidr > 32 {
        return Err(IsInRangeError {
            message: "CIDR must be an integer between 0 and 32".to_string(),
        });
    }

    let remote_long = umt_ip_to_long(remote_ip).map_err(|e| IsInRangeError {
        message: format!("Invalid IP address format: {}", e),
    })?;

    let network_long = umt_ip_to_long(network_ip).map_err(|e| IsInRangeError {
        message: format!("Invalid IP address format: {}", e),
    })?;

    // Special cases
    if cidr == 0 {
        return Ok(true); // All IPs are in range
    }
    if cidr == 32 {
        return Ok(remote_long == network_long); // Exact match required
    }

    // Normal case
    let shift = 32 - cidr;
    let mask = !0u32 << shift;
    Ok((remote_long & mask) == (network_long & mask))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_ranges() {
        // Common class C network
        assert_eq!(
            umt_is_in_range("192.168.1.2", "192.168.1.0", 24).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.2.2", "192.168.1.0", 24).unwrap(),
            false
        );

        // Class A network
        assert_eq!(umt_is_in_range("10.0.0.5", "10.0.0.0", 8).unwrap(), true);
        assert_eq!(umt_is_in_range("11.0.0.5", "10.0.0.0", 8).unwrap(), false);

        // Class B network
        assert_eq!(
            umt_is_in_range("172.16.1.1", "172.16.0.0", 16).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("172.17.1.1", "172.16.0.0", 16).unwrap(),
            false
        );

        // Single host network - exact match
        assert_eq!(
            umt_is_in_range("192.168.1.0", "192.168.1.0", 32).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.1", 32).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", 32).unwrap(),
            false
        );

        // Small subnet
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", 30).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.4", "192.168.1.0", 30).unwrap(),
            false
        );

        // All IPs in range (CIDR 0)
        assert_eq!(umt_is_in_range("0.0.0.1", "0.0.0.0", 0).unwrap(), true);
        assert_eq!(
            umt_is_in_range("255.255.255.255", "0.0.0.0", 0).unwrap(),
            true
        );
    }

    #[test]
    fn test_special_cidr_cases() {
        // CIDR 0: all IPs in range
        assert_eq!(
            umt_is_in_range("192.168.1.1", "0.0.0.0", 0).unwrap(),
            true
        );

        // CIDR 32: exact match only
        assert_eq!(
            umt_is_in_range("192.168.1.0", "192.168.1.0", 32).unwrap(),
            true
        );

        // CIDR 31: two IPs
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", 31).unwrap(),
            true
        );

        // CIDR 1: half of all IPs
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", 1).unwrap(),
            true
        );
    }

    #[test]
    fn test_edge_cases_with_specific_cidr_masks() {
        assert_eq!(
            umt_is_in_range("192.168.1.5", "192.168.1.0", 24).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.2.0", "192.168.1.0", 24).unwrap(),
            false
        );

        assert_eq!(
            umt_is_in_range("192.168.1.0", "192.168.1.0", 31).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", 31).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.2", "192.168.1.0", 31).unwrap(),
            false
        );

        assert_eq!(
            umt_is_in_range("192.168.1.3", "192.168.1.0", 30).unwrap(),
            true
        );
        assert_eq!(
            umt_is_in_range("192.168.1.4", "192.168.1.4", 30).unwrap(),
            true
        );
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(umt_is_in_range("", "192.168.1.0", 24).is_err());
        assert_eq!(
            umt_is_in_range("", "192.168.1.0", 24).unwrap_err().message,
            "Remote IP address is required"
        );

        assert!(umt_is_in_range("192.168.1.1", "", 24).is_err());
        assert_eq!(
            umt_is_in_range("192.168.1.1", "", 24).unwrap_err().message,
            "Network IP address is required"
        );

        assert!(umt_is_in_range("192.168.1.1", "192.168.1.0", -1).is_err());
        assert_eq!(
            umt_is_in_range("192.168.1.1", "192.168.1.0", -1)
                .unwrap_err()
                .message,
            "CIDR must be an integer between 0 and 32"
        );

        assert!(umt_is_in_range("192.168.1.1", "192.168.1.0", 33).is_err());
    }

    #[test]
    fn test_invalid_ip_formats() {
        assert!(umt_is_in_range("invalid", "192.168.1.0", 24).is_err());
        assert!(umt_is_in_range("192.168.1.1", "invalid", 24).is_err());
        assert!(umt_is_in_range("256.256.256.256", "192.168.1.0", 24).is_err());
        assert!(umt_is_in_range("192.168.1.1", "256.256.256.256", 24).is_err());
        assert!(umt_is_in_range("192.168.1.1.1", "192.168.1.0", 24).is_err());
        assert!(umt_is_in_range("192.168.-1.1", "192.168.1.0", 24).is_err());
        assert!(umt_is_in_range("192.168.1", "192.168.1.0", 24).is_err());
    }

    #[test]
    fn test_error_message_formatting() {
        let result = umt_is_in_range("invalid-ip", "192.168.1.0", 24);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("Invalid IP address format"));

        let result = umt_is_in_range("192.168.1.1", "invalid-network", 24);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("Invalid IP address format"));

        let result = umt_is_in_range("999.999.999.999", "192.168.1.0", 24);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("Invalid IP address format"));
    }
}
