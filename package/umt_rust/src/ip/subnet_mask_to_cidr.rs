/// Converts a subnet mask to CIDR notation
///
/// # Arguments
/// * `subnet_mask` - IPv4 subnet mask (e.g., "255.255.255.0")
///
/// # Returns
/// * `Ok(u8)` - CIDR notation (0-32)
/// * `Err` - If subnet mask is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::subnet_mask_to_cidr;
/// assert_eq!(subnet_mask_to_cidr("255.255.255.0").unwrap(), 24);
/// assert_eq!(subnet_mask_to_cidr("255.255.0.0").unwrap(), 16);
/// ```
pub fn subnet_mask_to_cidr(subnet_mask: &str) -> Result<u8, String> {
    if subnet_mask.is_empty() {
        return Err("Subnet mask is required".to_string());
    }

    // Parse octets and validate format
    let octets: Vec<&str> = subnet_mask.split('.').collect();
    if octets.len() != 4 {
        return Err("Invalid subnet mask format".to_string());
    }

    // Validate each octet and build binary string
    let mut binary_string = String::with_capacity(32);
    for octet in octets {
        let number: u8 = octet
            .parse()
            .map_err(|_| "Invalid subnet mask format".to_string())?;
        binary_string.push_str(&format!("{:08b}", number));
    }

    // Validate that mask is consecutive 1s followed by 0s
    let ones_count = binary_string.chars().take_while(|&c| c == '1').count();
    let zeros_start = binary_string.chars().skip(ones_count).all(|c| c == '0');

    if !zeros_start {
        return Err("Invalid subnet mask: must be consecutive 1s followed by 0s".to_string());
    }

    Ok(ones_count as u8)
}
