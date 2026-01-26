use umt_rust::math::{umt_random, umt_random_max};

#[test]
fn test_random_in_range() {
    for _ in 0..100 {
        let result = umt_random(10, 1);
        assert!(result >= 1 && result <= 10);
    }
}

#[test]
fn test_random_with_default_min() {
    for _ in 0..100 {
        let result = umt_random_max(5);
        assert!(result >= 0 && result <= 5);
    }
}

#[test]
fn test_random_same_min_max() {
    for _ in 0..10 {
        let result = umt_random(7, 7);
        assert_eq!(result, 7);
    }
}

#[test]
fn test_random_negative_range() {
    for _ in 0..100 {
        let result = umt_random(-5, -10);
        assert!(result >= -10 && result <= -5);
    }
}

#[test]
fn test_random_mixed_range() {
    for _ in 0..100 {
        let result = umt_random(5, -5);
        assert!(result >= -5 && result <= 5);
    }
}

#[test]
fn test_random_returns_integer() {
    for _ in 0..100 {
        let result = umt_random(100, 0);
        assert_eq!(result as f64, (result as f64).floor());
    }
}

use umt_rust::math::*;

#[test]
fn test_random_max_helper() {
    for _ in 0..100 {
        let r = umt_random_max(10);
        assert!(r >= 0 && r <= 10);
    }
}

#[test]
fn test_random_single_value() {
    for _ in 0..10 {
        let r = umt_random(5, 5);
        assert_eq!(r, 5);
    }
}

#[test]
fn test_random_with_min() {
    for _ in 0..100 {
        let r = umt_random(10, 5);
        assert!(r >= 5 && r <= 10);
    }
}
