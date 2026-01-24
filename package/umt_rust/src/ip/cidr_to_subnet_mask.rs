use crate::ip::cidr_to_long::cidr_to_long;
use crate::ip::long_to_ip::long_to_ip;

/// Converts CIDR notation to a subnet mask
///
/// # Arguments
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
/// * `Ok(String)` - Subnet mask in IPv4 format (e.g., "255.255.255.0")
/// * `Err` - If CIDR is not between 0 and 32
///
/// # Examples
/// ```
/// use umt_rust::ip::cidr_to_subnet_mask;
/// assert_eq!(cidr_to_subnet_mask(24).unwrap(), "255.255.255.0");
/// assert_eq!(cidr_to_subnet_mask(16).unwrap(), "255.255.0.0");
/// ```
pub fn cidr_to_subnet_mask(cidr: u8) -> Result<String, String> {
    let mask = cidr_to_long(cidr)?;
    Ok(long_to_ip(mask))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_to_subnet_mask() {
        assert_eq!(cidr_to_subnet_mask(0).unwrap(), "0.0.0.0");
        assert_eq!(cidr_to_subnet_mask(8).unwrap(), "255.0.0.0");
        assert_eq!(cidr_to_subnet_mask(16).unwrap(), "255.255.0.0");
        assert_eq!(cidr_to_subnet_mask(24).unwrap(), "255.255.255.0");
        assert_eq!(cidr_to_subnet_mask(32).unwrap(), "255.255.255.255");
    }

    #[test]
    fn test_cidr_to_subnet_mask_invalid() {
        assert!(cidr_to_subnet_mask(33).is_err());
    }
}
