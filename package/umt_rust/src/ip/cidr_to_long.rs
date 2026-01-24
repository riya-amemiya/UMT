/// Converts CIDR notation to a subnet mask number
///
/// # Arguments
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
/// * `Ok(u32)` - Subnet mask as a 32-bit number
/// * `Err` - If CIDR is not between 0 and 32
///
/// # Examples
/// ```
/// use umt_rust::ip::cidr_to_long;
/// assert_eq!(cidr_to_long(24).unwrap(), 0xFFFFFF00);
/// assert_eq!(cidr_to_long(32).unwrap(), 0xFFFFFFFF);
/// assert_eq!(cidr_to_long(0).unwrap(), 0);
/// ```
pub fn cidr_to_long(cidr: u8) -> Result<u32, String> {
    if cidr > 32 {
        return Err("CIDR must be an integer between 0 and 32".to_string());
    }

    if cidr == 0 {
        return Ok(0);
    }

    // Create a mask with `cidr` leading 1s
    Ok(!0u32 << (32 - cidr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_to_long_valid() {
        assert_eq!(cidr_to_long(0).unwrap(), 0);
        assert_eq!(cidr_to_long(8).unwrap(), 0xFF000000);
        assert_eq!(cidr_to_long(16).unwrap(), 0xFFFF0000);
        assert_eq!(cidr_to_long(24).unwrap(), 0xFFFFFF00);
        assert_eq!(cidr_to_long(32).unwrap(), 0xFFFFFFFF);
    }

    #[test]
    fn test_cidr_to_long_invalid() {
        assert!(cidr_to_long(33).is_err());
    }
}
