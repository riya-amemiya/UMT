use umt_rust::random::{umt_seeded_random, umt_seeded_random_from_str};

#[test]
fn test_produces_values_in_0_1() {
    let mut rand = umt_seeded_random(42);
    for _ in 0..100 {
        let value = rand();
        assert!(value >= 0.0);
        assert!(value < 1.0);
    }
}

#[test]
fn test_is_deterministic_for_the_same_numeric_seed() {
    let mut a = umt_seeded_random(123);
    let mut b = umt_seeded_random(123);
    for _ in 0..20 {
        assert_eq!(a(), b());
    }
}

#[test]
fn test_is_deterministic_for_the_same_string_seed() {
    let mut a = umt_seeded_random_from_str("hello");
    let mut b = umt_seeded_random_from_str("hello");
    for _ in 0..20 {
        assert_eq!(a(), b());
    }
}

#[test]
fn test_produces_different_streams_for_different_seeds() {
    let mut a = umt_seeded_random(1);
    let mut b = umt_seeded_random(2);
    let mut differs = false;
    for _ in 0..10 {
        if a() != b() {
            differs = true;
            break;
        }
    }
    assert!(differs);
}

#[test]
fn test_handles_a_numeric_seed_of_0() {
    let mut rand = umt_seeded_random(0);
    for _ in 0..20 {
        let value = rand();
        assert!(value >= 0.0);
        assert!(value < 1.0);
    }
}

#[test]
fn test_handles_an_empty_string_seed() {
    let mut rand = umt_seeded_random_from_str("");
    let value = rand();
    assert!(value >= 0.0);
    assert!(value < 1.0);
}
