use super::{umt_bitwise, RotateDirection};

/// Xoshiro256** state type.
pub type Xoshiro256State = [u32; 4];

/// Generates random numbers using the Xoshiro256** algorithm.
///
/// Xoshiro256** is a fast, high-quality pseudorandom number generator.
/// This implementation modifies the internal state array in place.
///
/// # Arguments
///
/// * `state` - Array of four 32-bit state values (modified in place).
/// * `min` - Minimum value of the generated random number (inclusive).
/// * `max` - Maximum value of the generated random number (exclusive).
///
/// # Returns
///
/// Generated random number between min and max.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_xoshiro256;
///
/// let mut state = [1u32, 2u32, 3u32, 4u32];
/// let result = umt_xoshiro256(&mut state, 0.0, 1.0);
/// assert!(result >= 0.0 && result < 1.0);
/// ```
pub fn umt_xoshiro256(state: &mut Xoshiro256State, min: f64, max: f64) -> f64 {
    let sum = state[0].wrapping_add(state[3]);
    let result = umt_bitwise(sum, 23, RotateDirection::Left).wrapping_add(state[0]);

    let t = state[1] << 17;

    state[2] ^= state[0];
    state[3] ^= state[1];
    state[1] ^= state[2];
    state[0] ^= state[3];

    state[2] ^= t;
    state[3] = umt_bitwise(state[3], 45, RotateDirection::Left);

    min + ((result as f64) / (u32::MAX as f64 + 1.0)) * (max - min)
}

/// Generates a random number between 0 and 1 using Xoshiro256**.
///
/// # Arguments
///
/// * `state` - Array of four 32-bit state values (modified in place).
///
/// # Returns
///
/// Generated random number between 0 (inclusive) and 1 (exclusive).
#[inline]
pub fn umt_xoshiro256_01(state: &mut Xoshiro256State) -> f64 {
    umt_xoshiro256(state, 0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoshiro256_range() {
        let mut state: Xoshiro256State = [1, 2, 3, 4];
        for _ in 0..100 {
            let result = umt_xoshiro256(&mut state, 0.0, 1.0);
            assert!(result >= 0.0 && result < 1.0);
        }
    }

    #[test]
    fn test_xoshiro256_custom_range() {
        let mut state: Xoshiro256State = [1, 2, 3, 4];
        for _ in 0..100 {
            let result = umt_xoshiro256(&mut state, 10.0, 20.0);
            assert!(result >= 10.0 && result < 20.0);
        }
    }

    #[test]
    fn test_xoshiro256_deterministic() {
        let mut state1: Xoshiro256State = [1, 2, 3, 4];
        let mut state2: Xoshiro256State = [1, 2, 3, 4];

        let result1 = umt_xoshiro256(&mut state1, 0.0, 1.0);
        let result2 = umt_xoshiro256(&mut state2, 0.0, 1.0);

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_xoshiro256_state_changes() {
        let mut state: Xoshiro256State = [1, 2, 3, 4];
        let initial_state = state;

        umt_xoshiro256(&mut state, 0.0, 1.0);

        assert_ne!(state, initial_state);
    }

    #[test]
    fn test_xoshiro256_01() {
        let mut state: Xoshiro256State = [1, 2, 3, 4];
        let result = umt_xoshiro256_01(&mut state);
        assert!(result >= 0.0 && result < 1.0);
    }
}
