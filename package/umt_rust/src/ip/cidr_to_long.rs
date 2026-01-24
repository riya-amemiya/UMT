/// Error type for CIDR conversion
#[derive(Debug, Clone, PartialEq)]
pub struct CidrError {
    pub message: String,
}

impl std::fmt::Display for CidrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CidrError {}

/// Converts CIDR notation to a subnet mask number.
///
/// # Arguments
///
/// * `cidr` - CIDR notation (0-32)
///
/// # Returns
///
/// A `Result` containing the subnet mask as a 32-bit unsigned integer,
/// or a `CidrError` if the CIDR value is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::ip::umt_cidr_to_long;
///
/// assert_eq!(umt_cidr_to_long(24).unwrap(), 0xFFFFFF00);
/// assert_eq!(umt_cidr_to_long(32).unwrap(), 0xFFFFFFFF);
/// assert_eq!(umt_cidr_to_long(0).unwrap(), 0x00000000);
/// ```
#[inline]
pub fn umt_cidr_to_long(cidr: i32) -> Result<u32, CidrError> {
    if cidr < 0 || cidr > 32 {
        return Err(CidrError {
            message: "CIDR must be an integer between 0 and 32".to_string(),
        });
    }

    if cidr == 0 {
        return Ok(0);
    }

    // Create a mask with `cidr` number of 1s followed by (32-cidr) 0s
    Ok(!0u32 << (32 - cidr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cidr_values() {
        // Common CIDR values
        assert_eq!(umt_cidr_to_long(32).unwrap(), 0xFFFFFFFF); // 255.255.255.255
        assert_eq!(umt_cidr_to_long(24).unwrap(), 0xFFFFFF00); // 255.255.255.0
        assert_eq!(umt_cidr_to_long(16).unwrap(), 0xFFFF0000); // 255.255.0.0
        assert_eq!(umt_cidr_to_long(8).unwrap(), 0xFF000000); // 255.0.0.0
        assert_eq!(umt_cidr_to_long(0).unwrap(), 0x00000000); // 0.0.0.0
        assert_eq!(umt_cidr_to_long(1).unwrap(), 0x80000000); // 128.0.0.0
        assert_eq!(umt_cidr_to_long(31).unwrap(), 0xFFFFFFFE); // 255.255.255.254
    }

    #[test]
    fn test_invalid_cidr_values() {
        assert!(umt_cidr_to_long(-1).is_err());
        assert_eq!(
            umt_cidr_to_long(-1).unwrap_err().message,
            "CIDR must be an integer between 0 and 32"
        );

        assert!(umt_cidr_to_long(33).is_err());
        assert_eq!(
            umt_cidr_to_long(33).unwrap_err().message,
            "CIDR must be an integer between 0 and 32"
        );
    }
}
