use regex::Regex;

/// Error type for IP to binary string conversion
#[derive(Debug, Clone, PartialEq)]
pub struct IpToBinaryError {
    pub message: String,
}

impl std::fmt::Display for IpToBinaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IpToBinaryError {}

/// Converts an IPv4 address to its binary string representation.
///
/// # Arguments
///
/// * `ip` - IPv4 address (e.g., "192.168.1.1")
///
/// # Returns
///
/// A `Result` containing the binary string representation (32 bits),
/// or an `IpToBinaryError` if the IP address is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_ip_to_binary_string;
///
/// assert_eq!(
///     umt_ip_to_binary_string("192.168.0.1").unwrap(),
///     "11000000101010000000000000000001"
/// );
/// ```
#[inline]
pub fn umt_ip_to_binary_string(ip: &str) -> Result<String, IpToBinaryError> {
    if ip.is_empty() {
        return Err(IpToBinaryError {
            message: "IP address is required".to_string(),
        });
    }

    // Check for invalid characters
    let invalid_chars = Regex::new(r"[^0-9.]").unwrap();
    if invalid_chars.is_match(ip) {
        return Err(IpToBinaryError {
            message: "Invalid IP address format".to_string(),
        });
    }

    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err(IpToBinaryError {
            message: "Invalid IP address format".to_string(),
        });
    }

    let mut binary_string = String::with_capacity(32);

    for octet_str in parts {
        // Check for empty octet or leading zeros
        if octet_str.is_empty() || (octet_str.len() > 1 && octet_str.starts_with('0')) {
            return Err(IpToBinaryError {
                message: "Invalid IP address format".to_string(),
            });
        }

        // Parse the octet
        let octet: u32 = octet_str.parse().map_err(|_| IpToBinaryError {
            message: "Invalid IP address format".to_string(),
        })?;

        if octet > 255 {
            return Err(IpToBinaryError {
                message: "Invalid IP address format".to_string(),
            });
        }

        // Convert to 8-bit binary string
        binary_string.push_str(&format!("{:08b}", octet));
    }

    Ok(binary_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_addresses() {
        assert_eq!(
            umt_ip_to_binary_string("192.168.0.1").unwrap(),
            "11000000101010000000000000000001"
        );
        assert_eq!(
            umt_ip_to_binary_string("0.0.0.0").unwrap(),
            "00000000000000000000000000000000"
        );
        assert_eq!(
            umt_ip_to_binary_string("255.255.255.255").unwrap(),
            "11111111111111111111111111111111"
        );
        assert_eq!(
            umt_ip_to_binary_string("1.2.3.4").unwrap(),
            "00000001000000100000001100000100"
        );
        assert_eq!(
            umt_ip_to_binary_string("10.0.0.1").unwrap(),
            "00001010000000000000000000000001"
        );
        assert_eq!(
            umt_ip_to_binary_string("172.16.0.1").unwrap(),
            "10101100000100000000000000000001"
        );
        assert_eq!(
            umt_ip_to_binary_string("127.0.0.1").unwrap(),
            "01111111000000000000000000000001"
        );
        assert_eq!(
            umt_ip_to_binary_string("169.254.0.1").unwrap(),
            "10101001111111100000000000000001"
        );
    }

    #[test]
    fn test_invalid_ip_addresses() {
        assert!(umt_ip_to_binary_string("").is_err());
        assert_eq!(
            umt_ip_to_binary_string("").unwrap_err().message,
            "IP address is required"
        );

        assert!(umt_ip_to_binary_string("192.168").is_err());
        assert!(umt_ip_to_binary_string("a.b.c.d").is_err());
        assert!(umt_ip_to_binary_string("256.1.2.3").is_err());
        assert!(umt_ip_to_binary_string("-1.1.1.1").is_err());
        assert!(umt_ip_to_binary_string("1.2.3.4.5").is_err());
        assert!(umt_ip_to_binary_string("192.168.1").is_err());
        assert!(umt_ip_to_binary_string("192.168.1.1.1").is_err());
        assert!(umt_ip_to_binary_string("192.168.1.").is_err());
        assert!(umt_ip_to_binary_string("192.168..1").is_err());
        assert!(umt_ip_to_binary_string(".192.168.1").is_err());
        assert!(umt_ip_to_binary_string("192,168,1,1").is_err());
        assert!(umt_ip_to_binary_string("192.168.1.1.").is_err());
        assert!(umt_ip_to_binary_string("192.168.1.+1").is_err());
        assert!(umt_ip_to_binary_string("256.256.256.256").is_err());
        assert!(umt_ip_to_binary_string("999.999.999.999").is_err());
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            umt_ip_to_binary_string("128.0.0.0").unwrap(),
            "10000000000000000000000000000000"
        );
        assert_eq!(
            umt_ip_to_binary_string("0.255.0.255").unwrap(),
            "00000000111111110000000011111111"
        );
        assert_eq!(
            umt_ip_to_binary_string("1.1.1.1").unwrap(),
            "00000001000000010000000100000001"
        );
    }

    #[test]
    fn test_reject_leading_zeros() {
        assert!(umt_ip_to_binary_string("0000").is_err());
        assert!(umt_ip_to_binary_string("00.00").is_err());
        assert!(umt_ip_to_binary_string("0.0.0").is_err());
        assert!(umt_ip_to_binary_string("192.168.01.1").is_err());
        assert!(umt_ip_to_binary_string("192.168.1.01").is_err());
        assert!(umt_ip_to_binary_string("010.020.030.040").is_err());
    }
}
