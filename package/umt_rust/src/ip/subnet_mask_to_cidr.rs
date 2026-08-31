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

    let mut mask = 0u32;
    let mut octet_count = 0u8;

    for octet in subnet_mask.split('.') {
        if octet_count == 4 {
            return Err("Invalid subnet mask format".to_string());
        }

        let number: u8 = octet
            .parse()
            .map_err(|_| "Invalid subnet mask format".to_string())?;

        mask = (mask << 8) | u32::from(number);
        octet_count += 1;
    }

    if octet_count != 4 {
        return Err("Invalid subnet mask format".to_string());
    }

    let host = !mask;
    if host & host.wrapping_add(1) != 0 {
        return Err("Invalid subnet mask: must be consecutive 1s followed by 0s".to_string());
    }

    Ok(mask.count_ones() as u8)
}
