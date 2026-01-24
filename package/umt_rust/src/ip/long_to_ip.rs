/// Converts a 32-bit number to an IPv4 address
///
/// # Arguments
/// * `long` - 32-bit unsigned integer to convert
///
/// # Returns
/// * IPv4 address string (e.g., "192.168.1.1")
///
/// # Examples
/// ```
/// use umt_rust::ip::long_to_ip;
/// assert_eq!(long_to_ip(3232235777), "192.168.1.1");
/// assert_eq!(long_to_ip(0), "0.0.0.0");
/// assert_eq!(long_to_ip(0xFFFFFFFF), "255.255.255.255");
/// ```
pub fn long_to_ip(long: u32) -> String {
    let octets = [
        (long >> 24) & 0xFF,
        (long >> 16) & 0xFF,
        (long >> 8) & 0xFF,
        long & 0xFF,
    ];

    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_to_ip() {
        assert_eq!(long_to_ip(0), "0.0.0.0");
        assert_eq!(long_to_ip(0xFFFFFFFF), "255.255.255.255");
        assert_eq!(long_to_ip(0xC0A80101), "192.168.1.1");
        assert_eq!(long_to_ip(0x0A000001), "10.0.0.1");
    }
}
