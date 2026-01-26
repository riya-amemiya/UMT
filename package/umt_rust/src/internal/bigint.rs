/// A simple big unsigned integer represented as a vector of base-2^32 digits (little-endian).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUint {
    digits: Vec<u32>,
}

impl BigUint {
    pub fn zero() -> Self {
        BigUint { digits: vec![0] }
    }

    pub fn from_u32(val: u32) -> Self {
        BigUint { digits: vec![val] }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.iter().all(|&d| d == 0)
    }

    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::zero();
        }

        // Process bytes in groups of 4 to create u32 digits (big-endian to little-endian)
        let mut result = Self::zero();
        for &byte in bytes {
            result = result.mul_u32(256).add_u32(byte as u32);
        }
        result
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        if self.is_zero() {
            return vec![0];
        }

        let mut bytes = Vec::new();
        let mut n = self.clone();
        while !n.is_zero() {
            let (quotient, remainder) = n.div_mod_u32(256);
            bytes.push(remainder as u8);
            n = quotient;
        }
        bytes.reverse();
        bytes
    }

    pub fn div_mod_u32(&self, divisor: u32) -> (BigUint, u32) {
        assert!(divisor != 0, "Division by zero");

        let divisor = divisor as u64;
        let mut quotient_digits = vec![0u32; self.digits.len()];
        let mut remainder: u64 = 0;

        for i in (0..self.digits.len()).rev() {
            let current = remainder * (1u64 << 32) + self.digits[i] as u64;
            quotient_digits[i] = (current / divisor) as u32;
            remainder = current % divisor;
        }

        // Remove leading zeros
        while quotient_digits.len() > 1 && *quotient_digits.last().unwrap() == 0 {
            quotient_digits.pop();
        }

        (
            BigUint {
                digits: quotient_digits,
            },
            remainder as u32,
        )
    }

    pub fn mul_u32(&self, multiplier: u32) -> BigUint {
        let multiplier = multiplier as u64;
        let mut result_digits = vec![0u32; self.digits.len() + 1];
        let mut carry: u64 = 0;

        for i in 0..self.digits.len() {
            let product = self.digits[i] as u64 * multiplier + carry;
            result_digits[i] = product as u32;
            carry = product >> 32;
        }

        if carry > 0 {
            result_digits[self.digits.len()] = carry as u32;
        }

        // Remove leading zeros
        while result_digits.len() > 1 && *result_digits.last().unwrap() == 0 {
            result_digits.pop();
        }

        BigUint {
            digits: result_digits,
        }
    }

    pub fn add_u32(&self, addend: u32) -> BigUint {
        let mut result_digits = self.digits.clone();
        let mut carry = addend as u64;

        for digit in result_digits.iter_mut() {
            let sum = *digit as u64 + carry;
            *digit = sum as u32;
            carry = sum >> 32;
            if carry == 0 {
                break;
            }
        }

        if carry > 0 {
            result_digits.push(carry as u32);
        }

        BigUint {
            digits: result_digits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert!(BigUint::zero().is_zero());
        assert!(!BigUint::from_u32(1).is_zero());
    }

    #[test]
    fn test_from_bytes_be() {
        let n = BigUint::from_bytes_be(&[72, 101, 108, 108, 111]); // "Hello"
        assert!(!n.is_zero());
    }

    #[test]
    fn test_roundtrip_bytes() {
        let original = vec![255, 254, 253, 252, 251];
        let n = BigUint::from_bytes_be(&original);
        let result = n.to_bytes_be();
        assert_eq!(result, original);
    }

    #[test]
    fn test_div_mod() {
        let n = BigUint::from_u32(100);
        let (q, r) = n.div_mod_u32(58);
        assert_eq!(q, BigUint::from_u32(1));
        assert_eq!(r, 42);
    }

    #[test]
    fn test_mul_add() {
        let n = BigUint::from_u32(10);
        let result = n.mul_u32(58).add_u32(5);
        // 10 * 58 + 5 = 585
        let (q, r) = result.div_mod_u32(585);
        assert_eq!(r, 0);
        assert_eq!(q, BigUint::from_u32(1));
    }
}
