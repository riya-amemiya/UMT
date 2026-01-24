/// Converts an IPv4 address to its binary string representation
///
/// # Arguments
/// * `ip` - IPv4 address (e.g., "192.168.1.1")
///
/// # Returns
/// * `Ok(String)` - Binary string representation (32 bits)
/// * `Err` - If IP address is invalid
///
/// # Examples
/// ```
/// use umt_rust::ip::ip_to_binary_string;
/// assert_eq!(ip_to_binary_string("192.168.1.1").unwrap(), "11000000101010000000000100000001");
/// ```
pub fn ip_to_binary_string(ip: &str) -> Result<String, String> {
    if ip.is_empty() {
        return Err("IP address is required".to_string());
    }

    // Check for invalid characters
    if ip.chars().any(|c| !c.is_ascii_digit() && c != '.') {
        return Err("Invalid IP address format".to_string());
    }

    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err("Invalid IP address format".to_string());
    }

    let mut binary_parts = Vec::with_capacity(4);

    for octet in parts {
        // Check for empty octet or leading zeros
        if octet.is_empty() || (octet.len() > 1 && octet.starts_with('0')) {
            return Err("Invalid IP address format".to_string());
        }

        let number: u8 = octet
            .parse()
            .map_err(|_| "Invalid IP address format".to_string())?;

        binary_parts.push(format!("{:08b}", number));
    }

    Ok(binary_parts.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_to_binary_string_valid() {
        assert_eq!(
            ip_to_binary_string("192.168.1.1").unwrap(),
            "11000000101010000000000100000001"
        );
        assert_eq!(
            ip_to_binary_string("0.0.0.0").unwrap(),
            "00000000000000000000000000000000"
        );
        assert_eq!(
            ip_to_binary_string("255.255.255.255").unwrap(),
            "11111111111111111111111111111111"
        );
    }

    #[test]
    fn test_ip_to_binary_string_invalid() {
        assert!(ip_to_binary_string("").is_err());
        assert!(ip_to_binary_string("192.168.1").is_err());
        assert!(ip_to_binary_string("192.168.1.1.1").is_err());
        assert!(ip_to_binary_string("192.168.01.1").is_err());
        assert!(ip_to_binary_string("192.168.1.256").is_err());
        assert!(ip_to_binary_string("abc.def.ghi.jkl").is_err());
    }
}
