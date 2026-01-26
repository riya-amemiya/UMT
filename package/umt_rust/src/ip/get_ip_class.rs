/// Gets the IP address class (A, B, C, D, or E)
///
/// # Arguments
/// * `ip` - IPv4 address
///
/// # Returns
/// * IP class ('A', 'B', 'C', 'D', 'E', or empty string for invalid IP)
///
/// # Examples
/// ```
/// use umt_rust::ip::get_ip_class;
/// assert_eq!(get_ip_class("10.0.0.1"), "A");
/// assert_eq!(get_ip_class("172.16.0.1"), "B");
/// assert_eq!(get_ip_class("192.168.1.1"), "C");
/// ```
pub fn get_ip_class(ip: &str) -> &'static str {
    if ip.is_empty() {
        return "";
    }

    // Validate IP format
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return "";
    }

    let first_octet: u8 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return "",
    };

    // Check each class range
    match first_octet {
        0 => "",          // Reserved
        1..=127 => "A",   // Class A
        128..=191 => "B", // Class B
        192..=223 => "C", // Class C
        224..=239 => "D", // Class D (Multicast)
        240..=255 => "E", // Class E (Reserved)
    }
}
