use crate::math::{Xoshiro256State, umt_xoshiro256_01};

/// FNV-1a 32-bit hash of a string's UTF-8 bytes.
///
/// Matches the TypeScript `hashStringToSeed`, which encodes the input with
/// `TextEncoder` (UTF-8) and folds each byte with the FNV-1a constants.
fn hash_string_to_seed(input: &str) -> u32 {
    let mut hash: u32 = 2_166_136_261;
    for byte in input.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16_777_619);
    }
    hash
}

/// Builds the initial xoshiro256** state from a 32-bit seed.
///
/// Mirrors the TypeScript state initialization, where each lane is the
/// multiplied seed (low 32 bits), falling back to `1..=4` when a lane would be
/// zero.
fn build_state(initial: u32) -> Xoshiro256State {
    [
        if initial == 0 { 1 } else { initial },
        {
            let v = initial.wrapping_mul(0x9e_37_79_b9);
            if v == 0 { 2 } else { v }
        },
        {
            let v = initial.wrapping_mul(0x85_eb_ca_6b);
            if v == 0 { 3 } else { v }
        },
        {
            let v = initial.wrapping_mul(0xc2_b2_ae_35);
            if v == 0 { 4 } else { v }
        },
    ]
}

/// Returns a deterministic PRNG seeded from a numeric seed.
///
/// Each call to the returned closure produces a float in `[0, 1)` from the same
/// xoshiro256** stream. Identical seeds yield identical streams.
///
/// # Arguments
///
/// * `seed` - Numeric seed value (truncated to its low 32 bits).
///
/// # Returns
///
/// A closure that returns sequential pseudo-random floats in `[0, 1)`.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_seeded_random;
///
/// let mut rand = umt_seeded_random(42);
/// let value = rand();
/// assert!(value >= 0.0 && value < 1.0);
///
/// let mut a = umt_seeded_random(123);
/// let mut b = umt_seeded_random(123);
/// assert_eq!(a(), b());
/// ```
pub fn umt_seeded_random(seed: u32) -> impl FnMut() -> f64 {
    let mut state = build_state(seed);
    move || umt_xoshiro256_01(&mut state)
}

/// Returns a deterministic PRNG seeded from a string.
///
/// The string is hashed with FNV-1a to derive the 32-bit seed, then the stream
/// behaves identically to [`umt_seeded_random`].
///
/// # Arguments
///
/// * `seed` - String seed value.
///
/// # Returns
///
/// A closure that returns sequential pseudo-random floats in `[0, 1)`.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_seeded_random_from_str;
///
/// let mut a = umt_seeded_random_from_str("hello");
/// let mut b = umt_seeded_random_from_str("hello");
/// assert_eq!(a(), b());
/// ```
pub fn umt_seeded_random_from_str(seed: &str) -> impl FnMut() -> f64 {
    let mut state = build_state(hash_string_to_seed(seed));
    move || umt_xoshiro256_01(&mut state)
}
