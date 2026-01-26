use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

thread_local! {
    static RNG_STATE: RefCell<[u64; 4]> = RefCell::new(init_state());
}

fn init_state() -> [u64; 4] {
    let mut state = [0u64; 4];
    for s in state.iter_mut() {
        let rs = RandomState::new();
        let mut h = rs.build_hasher();
        h.write_u64(0);
        *s = h.finish();
    }
    // Ensure state is not all zeros (xoshiro requirement)
    if state.iter().all(|&s| s == 0) {
        state[0] = 1;
    }
    state
}

/// Xoshiro256** PRNG - generates a random u64
fn xoshiro256ss(state: &mut [u64; 4]) -> u64 {
    let result = state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
    let t = state[1] << 17;
    state[2] ^= state[0];
    state[3] ^= state[1];
    state[1] ^= state[2];
    state[0] ^= state[3];
    state[2] ^= t;
    state[3] = state[3].rotate_left(45);
    result
}

/// Generate a random u64
pub fn random_u64() -> u64 {
    RNG_STATE.with(|state| {
        let mut state = state.borrow_mut();
        xoshiro256ss(&mut state)
    })
}

/// Generate a random u32
pub fn random_u32() -> u32 {
    random_u64() as u32
}

/// Generate a random f64 in [0, 1)
pub fn random_f64() -> f64 {
    (random_u64() >> 11) as f64 / (1u64 << 53) as f64
}

/// Generate a random i64 in [min, max] (inclusive)
pub fn random_range_i64(min: i64, max: i64) -> i64 {
    if min == max {
        return min;
    }
    let range = (max as u64).wrapping_sub(min as u64).wrapping_add(1);
    if range == 0 {
        // Full range
        return random_u64() as i64;
    }
    min.wrapping_add((random_u64() % range) as i64)
}

/// Generate a random usize in [min, max] (inclusive)
pub fn random_range_usize(min: usize, max: usize) -> usize {
    if min == max {
        return min;
    }
    let range = (max - min + 1) as u64;
    min + (random_u64() % range) as usize
}

/// Generate a random u16 in [0, max) (exclusive)
pub fn random_u16_below(max: u16) -> u16 {
    (random_u64() % max as u64) as u16
}

/// Generate a random u32 in [0, max) (exclusive)
pub fn random_u32_below(max: u32) -> u32 {
    (random_u64() % max as u64) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_u64() {
        let a = random_u64();
        let b = random_u64();
        // Extremely unlikely to be equal
        assert_ne!(a, b);
    }

    #[test]
    fn test_random_f64_range() {
        for _ in 0..100 {
            let v = random_f64();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn test_random_range_i64() {
        for _ in 0..100 {
            let v = random_range_i64(5, 10);
            assert!(v >= 5 && v <= 10);
        }
    }

    #[test]
    fn test_random_range_usize() {
        for _ in 0..100 {
            let v = random_range_usize(0, 5);
            assert!(v <= 5);
        }
    }

    #[test]
    fn test_random_range_single() {
        assert_eq!(random_range_i64(5, 5), 5);
        assert_eq!(random_range_usize(3, 3), 3);
    }
}
