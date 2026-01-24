use super::cidr_to_long::{umt_cidr_to_long, CidrError};
use super::long_to_ip::umt_long_to_ip;

/// Converts CIDR notation to a subnet mask string.
///
/// # Arguments
///
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
///
/// A `Result` containing the subnet mask in IPv4 format (e.g., "255.255.255.0"),
/// or a `CidrError` if the CIDR value is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_cidr_to_subnet_mask;
///
/// assert_eq!(umt_cidr_to_subnet_mask(24).unwrap(), "255.255.255.0");
/// assert_eq!(umt_cidr_to_subnet_mask(16).unwrap(), "255.255.0.0");
/// ```
#[inline]
pub fn umt_cidr_to_subnet_mask(cidr: i32) -> Result<String, CidrError> {
    let long = umt_cidr_to_long(cidr)?;
    // long_to_ip never fails for valid u32 values, so unwrap is safe here
    Ok(umt_long_to_ip(long).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cidr_values() {
        assert_eq!(umt_cidr_to_subnet_mask(32).unwrap(), "255.255.255.255"); // Full mask
        assert_eq!(umt_cidr_to_subnet_mask(24).unwrap(), "255.255.255.0"); // Common class C
        assert_eq!(umt_cidr_to_subnet_mask(16).unwrap(), "255.255.0.0"); // Common class B
        assert_eq!(umt_cidr_to_subnet_mask(8).unwrap(), "255.0.0.0"); // Common class A
        assert_eq!(umt_cidr_to_subnet_mask(0).unwrap(), "0.0.0.0"); // No mask
        assert_eq!(umt_cidr_to_subnet_mask(1).unwrap(), "128.0.0.0"); // First bit
        assert_eq!(umt_cidr_to_subnet_mask(31).unwrap(), "255.255.255.254"); // Second to last
        assert_eq!(umt_cidr_to_subnet_mask(28).unwrap(), "255.255.255.240"); // Common subnet
        assert_eq!(umt_cidr_to_subnet_mask(20).unwrap(), "255.255.240.0"); // Less common subnet
    }

    #[test]
    fn test_invalid_cidr_values() {
        assert!(umt_cidr_to_subnet_mask(-1).is_err());
        assert_eq!(
            umt_cidr_to_subnet_mask(-1).unwrap_err().message,
            "CIDR must be an integer between 0 and 32"
        );

        assert!(umt_cidr_to_subnet_mask(33).is_err());
    }
}
