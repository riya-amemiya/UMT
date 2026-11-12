/// Reads the leading integer of an octet the way JavaScript `parseInt(octet, 10)` does:
/// leading whitespace and a sign are accepted, and parsing stops at the first
/// character that is not an ASCII digit. Values above 255 saturate.
fn leading_octet(octet: &str) -> Option<u32> {
    let trimmed = octet.trim_start_matches(|character: char| {
        matches!(
            character,
            '\t' | '\n'
                | '\u{000B}'
                | '\u{000C}'
                | '\r'
                | ' '
                | '\u{00A0}'
                | '\u{1680}'
                | '\u{2000}'
                ..='\u{200A}'
                    | '\u{2028}'
                    | '\u{2029}'
                    | '\u{202F}'
                    | '\u{205F}'
                    | '\u{3000}'
                    | '\u{FEFF}'
        )
    });
    let negative = trimmed.starts_with('-');
    let unsigned = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
    let digit_count = unsigned.bytes().take_while(u8::is_ascii_digit).count();
    if digit_count == 0 {
        return None;
    }

    let value = unsigned.as_bytes()[..digit_count]
        .iter()
        .fold(0u32, |accumulator, &digit| {
            accumulator
                .saturating_mul(10)
                .saturating_add(u32::from(digit - b'0'))
        });
    if negative && value != 0 {
        return None;
    }
    Some(value)
}

/// Gets the IP address class (A, B, C, D, or E)
///
/// The first octet is read like JavaScript `parseInt(octet, 10)`, so `"10abc.0.0.1"`
/// is class A.
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
    let mut parts = ip.split('.');
    let first = parts.next().unwrap_or("");
    if parts.count() != 3 {
        return "";
    }

    // Check each class range
    match leading_octet(first) {
        Some(1..=127) => "A",   // Class A
        Some(128..=191) => "B", // Class B
        Some(192..=223) => "C", // Class C
        Some(224..=239) => "D", // Class D (Multicast)
        Some(240..=255) => "E", // Class E (Reserved)
        _ => "",                // Reserved 0, out of range, or not a number
    }
}
