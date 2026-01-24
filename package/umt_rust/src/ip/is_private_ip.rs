use super::is_in_range::umt_is_in_range;

/// Error type for private IP checking
#[derive(Debug, Clone, PartialEq)]
pub struct IsPrivateIpError {
    pub message: String,
}

impl std::fmt::Display for IsPrivateIpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IsPrivateIpError {}

/// Private IP range definition
struct PrivateRange {
    network: &'static str,
    cidr: i32,
}

/// Checks if an IP address is within private IP ranges.
///
/// Private IP ranges:
/// - Class A: 10.0.0.0/8
/// - Class B: 172.16.0.0/12
/// - Class C: 192.168.0.0/16
///
/// # Arguments
///
/// * `ip` - IP address to check (e.g., "192.168.1.1")
///
/// # Returns
///
/// A `Result` containing a boolean indicating whether the IP is private,
/// or an `IsPrivateIpError` if the IP address is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_is_private_ip;
///
/// assert_eq!(umt_is_private_ip("192.168.1.1").unwrap(), true);
/// assert_eq!(umt_is_private_ip("8.8.8.8").unwrap(), false);
/// ```
#[inline]
pub fn umt_is_private_ip(ip: &str) -> Result<bool, IsPrivateIpError> {
    if ip.is_empty() {
        return Err(IsPrivateIpError {
            message: "IP address is required".to_string(),
        });
    }

    // Define private IP ranges with their CIDR notations
    let private_ranges = [
        PrivateRange {
            network: "10.0.0.0",
            cidr: 8,
        }, // Class A private network
        PrivateRange {
            network: "172.16.0.0",
            cidr: 12,
        }, // Class B private network
        PrivateRange {
            network: "192.168.0.0",
            cidr: 16,
        }, // Class C private network
    ];

    for range in &private_ranges {
        match umt_is_in_range(ip, range.network, range.cidr) {
            Ok(true) => return Ok(true),
            Ok(false) => continue,
            Err(e) => {
                return Err(IsPrivateIpError {
                    message: format!("Invalid IP address: {}", e),
                })
            }
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_ip_addresses() {
        // Class A private
        assert_eq!(umt_is_private_ip("10.0.0.1").unwrap(), true); // start range
        assert_eq!(umt_is_private_ip("10.255.255.255").unwrap(), true); // end range

        // Class B private
        assert_eq!(umt_is_private_ip("172.16.0.1").unwrap(), true); // start range
        assert_eq!(umt_is_private_ip("172.31.255.255").unwrap(), true); // end range

        // Class C private
        assert_eq!(umt_is_private_ip("192.168.0.1").unwrap(), true); // start range
        assert_eq!(umt_is_private_ip("192.168.255.255").unwrap(), true); // end range
    }

    #[test]
    fn test_non_private_ip_addresses() {
        assert_eq!(umt_is_private_ip("9.255.255.255").unwrap(), false); // Just before Class A private
        assert_eq!(umt_is_private_ip("11.0.0.0").unwrap(), false); // Just after Class A private
        assert_eq!(umt_is_private_ip("172.15.255.255").unwrap(), false); // Just before Class B private
        assert_eq!(umt_is_private_ip("172.32.0.0").unwrap(), false); // Just after Class B private
        assert_eq!(umt_is_private_ip("192.167.255.255").unwrap(), false); // Just before Class C private
        assert_eq!(umt_is_private_ip("192.169.0.0").unwrap(), false); // Just after Class C private
        assert_eq!(umt_is_private_ip("8.8.8.8").unwrap(), false); // Google DNS
        assert_eq!(umt_is_private_ip("1.1.1.1").unwrap(), false); // Cloudflare DNS
        assert_eq!(umt_is_private_ip("169.254.0.1").unwrap(), false); // Link-local address
        assert_eq!(umt_is_private_ip("127.0.0.1").unwrap(), false); // Localhost
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(umt_is_private_ip("").is_err());
        assert_eq!(
            umt_is_private_ip("").unwrap_err().message,
            "IP address is required"
        );

        assert!(umt_is_private_ip("256.256.256.256").is_err());
        assert!(umt_is_private_ip("192.168").is_err());
        assert!(umt_is_private_ip("a.b.c.d").is_err());
        assert!(umt_is_private_ip("192.168.1.1.1").is_err());
        assert!(umt_is_private_ip("-1.0.0.0").is_err());
    }
}
