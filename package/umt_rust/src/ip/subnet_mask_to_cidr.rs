use regex::Regex;

/// Error type for subnet mask to CIDR conversion
#[derive(Debug, Clone, PartialEq)]
pub struct SubnetMaskError {
    pub message: String,
}

impl std::fmt::Display for SubnetMaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SubnetMaskError {}

/// Converts a subnet mask to CIDR notation.
///
/// # Arguments
///
/// * `subnet_mask` - IPv4 subnet mask (e.g., "255.255.255.0")
///
/// # Returns
///
/// A `Result` containing the CIDR notation (0-32),
/// or a `SubnetMaskError` if the subnet mask is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_subnet_mask_to_cidr;
///
/// assert_eq!(umt_subnet_mask_to_cidr("255.255.255.0").unwrap(), 24);
/// assert_eq!(umt_subnet_mask_to_cidr("255.255.0.0").unwrap(), 16);
/// ```
#[inline]
pub fn umt_subnet_mask_to_cidr(subnet_mask: &str) -> Result<i32, SubnetMaskError> {
    if subnet_mask.is_empty() {
        return Err(SubnetMaskError {
            message: "Subnet mask is required".to_string(),
        });
    }

    // Parse octets and validate format
    let octets: Vec<&str> = subnet_mask.split('.').collect();
    if octets.len() != 4 {
        return Err(SubnetMaskError {
            message: "Invalid subnet mask format".to_string(),
        });
    }

    // Build binary string from octets
    let mut binary_string = String::with_capacity(32);
    for octet_str in octets {
        let octet: i32 = octet_str.parse().map_err(|_| SubnetMaskError {
            message: "Invalid subnet mask format".to_string(),
        })?;

        if octet < 0 || octet > 255 {
            return Err(SubnetMaskError {
                message: "Invalid subnet mask format".to_string(),
            });
        }

        binary_string.push_str(&format!("{:08b}", octet));
    }

    // Validate that the mask is consecutive 1s followed by 0s
    let valid_pattern = Regex::new(r"^1*0*$").unwrap();
    if !valid_pattern.is_match(&binary_string) {
        return Err(SubnetMaskError {
            message: "Invalid subnet mask: must be consecutive 1s followed by 0s".to_string(),
        });
    }

    // Count 1s
    let cidr = binary_string.chars().filter(|&c| c == '1').count() as i32;
    Ok(cidr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_subnet_masks() {
        assert_eq!(umt_subnet_mask_to_cidr("255.255.255.255").unwrap(), 32); // All bits set
        assert_eq!(umt_subnet_mask_to_cidr("255.255.255.0").unwrap(), 24); // Common /24 network
        assert_eq!(umt_subnet_mask_to_cidr("255.255.0.0").unwrap(), 16); // Common /16 network
        assert_eq!(umt_subnet_mask_to_cidr("255.0.0.0").unwrap(), 8); // Common /8 network
        assert_eq!(umt_subnet_mask_to_cidr("255.255.254.0").unwrap(), 23); // Less common /23 network
        assert_eq!(umt_subnet_mask_to_cidr("255.255.255.252").unwrap(), 30); // Small /30 network
        assert_eq!(umt_subnet_mask_to_cidr("255.255.255.248").unwrap(), 29); // Small /29 network
        assert_eq!(umt_subnet_mask_to_cidr("255.255.240.0").unwrap(), 20); // Medium /20 network
        assert_eq!(umt_subnet_mask_to_cidr("0.0.0.0").unwrap(), 0); // No bits set
    }

    #[test]
    fn test_invalid_subnet_masks() {
        assert!(umt_subnet_mask_to_cidr("").is_err());
        assert_eq!(
            umt_subnet_mask_to_cidr("").unwrap_err().message,
            "Subnet mask is required"
        );

        assert!(umt_subnet_mask_to_cidr("192.168").is_err());
        assert!(umt_subnet_mask_to_cidr("256.255.255.0").is_err());
        assert!(umt_subnet_mask_to_cidr("255.255.255.256").is_err());
        assert!(umt_subnet_mask_to_cidr("255.-1.255.0").is_err());
        assert!(umt_subnet_mask_to_cidr("255.255.255.abc").is_err());
        assert!(umt_subnet_mask_to_cidr("a.b.c.d").is_err());
    }

    #[test]
    fn test_invalid_subnet_mask_patterns() {
        // Non-continuous masks
        assert!(umt_subnet_mask_to_cidr("255.255.255.1").is_err());
        assert_eq!(
            umt_subnet_mask_to_cidr("255.255.255.1").unwrap_err().message,
            "Invalid subnet mask: must be consecutive 1s followed by 0s"
        );

        assert!(umt_subnet_mask_to_cidr("255.0.255.0").is_err());
        assert!(umt_subnet_mask_to_cidr("254.255.255.0").is_err());
    }
}
