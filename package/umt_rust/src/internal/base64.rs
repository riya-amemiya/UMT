const ENCODE_TABLE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const DECODE_TABLE: [u8; 256] = {
    let mut table = [255u8; 256];
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut i = 0;
    while i < 64 {
        table[chars[i] as usize] = i as u8;
        i += 1;
    }
    table
};

pub fn encode(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let output_len = 4 * ((data.len() + 2) / 3);
    let mut result = String::with_capacity(output_len);

    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(ENCODE_TABLE[((triple >> 18) & 0x3F) as usize] as char);
        result.push(ENCODE_TABLE[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(ENCODE_TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ENCODE_TABLE[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let input = input.as_bytes();
    if input.len() % 4 != 0 {
        return Err("Invalid Base64 length");
    }

    let mut result = Vec::with_capacity(input.len() / 4 * 3);

    for chunk in input.chunks(4) {
        let mut values = [0u8; 4];
        let mut padding = 0;

        for (i, &byte) in chunk.iter().enumerate() {
            if byte == b'=' {
                padding += 1;
                values[i] = 0;
            } else {
                let val = DECODE_TABLE[byte as usize];
                if val == 255 {
                    return Err("Invalid Base64 character");
                }
                values[i] = val;
            }
        }

        let triple =
            ((values[0] as u32) << 18) | ((values[1] as u32) << 12) | ((values[2] as u32) << 6) | (values[3] as u32);

        result.push(((triple >> 16) & 0xFF) as u8);
        if padding < 2 {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if padding < 1 {
            result.push((triple & 0xFF) as u8);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        assert_eq!(encode(b"Hello World"), "SGVsbG8gV29ybGQ=");
        assert_eq!(encode(b"Hello, World!"), "SGVsbG8sIFdvcmxkIQ==");
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_decode_basic() {
        assert_eq!(decode("SGVsbG8gV29ybGQ=").unwrap(), b"Hello World");
        assert_eq!(decode("SGVsbG8sIFdvcmxkIQ==").unwrap(), b"Hello, World!");
        assert_eq!(decode("").unwrap(), b"");
    }

    #[test]
    fn test_roundtrip() {
        let original = b"The quick brown fox jumps over the lazy dog";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_invalid() {
        assert!(decode("!!!").is_err());
    }
}
