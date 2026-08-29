/// Converts an IPv4 address to a 32-bit number
///
/// # Arguments
/// * `ip` - IPv4 address to convert (e.g., "192.168.1.1")
///
/// # Returns
/// * `Ok(u32)` - 32-bit unsigned integer
/// * `Err` - If IP address is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::ip_to_long;
/// assert_eq!(ip_to_long("192.168.1.1").unwrap(), 3232235777);
/// ```
pub fn ip_to_long(ip: &str) -> Result<u32, String> {
    if ip.is_empty() {
        return Err("IP address is required".to_string());
    }

    let mut result = 0u32;
    let mut octet_count = 0u8;

    for octet in ip.split('.') {
        if octet_count == 4
            || octet.is_empty()
            || !octet.bytes().all(|b| b.is_ascii_digit())
            || (octet.len() > 1 && octet.starts_with('0'))
        {
            return Err("Invalid IP address format".to_string());
        }

        let number: u8 = octet
            .parse()
            .map_err(|_| "Invalid IP address format".to_string())?;

        result = (result << 8) | u32::from(number);
        octet_count += 1;
    }

    if octet_count != 4 {
        return Err("Invalid IP address format".to_string());
    }

    Ok(result)
}
