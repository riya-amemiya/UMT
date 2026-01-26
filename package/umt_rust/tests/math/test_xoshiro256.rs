use umt_rust::math::{Xoshiro256State, umt_xoshiro256, umt_xoshiro256_01};

#[test]
fn test_xoshiro256_default_range() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    let result = umt_xoshiro256(&mut state, 0.0, 1.0);
    assert!(result >= 0.0);
    assert!(result < 1.0);
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
fn test_xoshiro256_different_seeds() {
    let mut state1: Xoshiro256State = [1, 2, 3, 4];
    let mut state2: Xoshiro256State = [4, 3, 2, 1];
    let result1 = umt_xoshiro256(&mut state1, 0.0, 1.0);
    let result2 = umt_xoshiro256(&mut state2, 0.0, 1.0);
    assert_ne!(result1, result2);
}

#[test]
fn test_xoshiro256_custom_range() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    let min = 10.0;
    let max = 20.0;
    let result = umt_xoshiro256(&mut state, min, max);
    assert!(result >= min);
    assert!(result < max);
}

#[test]
fn test_xoshiro256_01_helper() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    let result = umt_xoshiro256_01(&mut state);
    assert!(result >= 0.0);
    assert!(result < 1.0);
}

#[test]
fn test_xoshiro256_state_changes() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    let initial_state = state;
    umt_xoshiro256(&mut state, 0.0, 1.0);
    assert_ne!(state, initial_state);
}

#[test]
fn test_xoshiro256_many_iterations() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    for _ in 0..100 {
        let result = umt_xoshiro256(&mut state, 0.0, 1.0);
        assert!(result >= 0.0 && result < 1.0);
    }
}

#[test]
fn test_xoshiro256_custom_range_iterations() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    for _ in 0..100 {
        let result = umt_xoshiro256(&mut state, 10.0, 20.0);
        assert!(result >= 10.0 && result < 20.0);
    }
}

use umt_rust::math::*;

#[test]
fn test_xoshiro256_01() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    let result = umt_xoshiro256_01(&mut state);
    assert!(result >= 0.0 && result < 1.0);
}

#[test]
fn test_xoshiro256_range() {
    let mut state: Xoshiro256State = [1, 2, 3, 4];
    for _ in 0..100 {
        let result = umt_xoshiro256(&mut state, 0.0, 1.0);
        assert!(result >= 0.0 && result < 1.0);
    }
}
