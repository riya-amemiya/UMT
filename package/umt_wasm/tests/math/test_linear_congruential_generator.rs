use umtn_wasm::linear_congruential_generator;

#[test]
fn test_seed_i32_min() {
    assert!(std::panic::catch_unwind(|| {
        linear_congruential_generator(i32::MIN, None, None, None)
    })
    .is_err());
}

#[test]
fn test_seed_minus_one() {
    assert_eq!(
        linear_congruential_generator(-1, None, None, None),
        1012239698
    );
}

#[test]
fn test_seed_zero() {
    assert_eq!(
        linear_congruential_generator(0, None, None, None),
        1013904223
    );
}

#[test]
fn test_seed_one() {
    assert_eq!(
        linear_congruential_generator(1, None, None, None),
        1015568748
    );
}

#[test]
fn test_seed_i32_max() {
    assert!(std::panic::catch_unwind(|| {
        linear_congruential_generator(i32::MAX, None, None, None)
    })
    .is_err());
}

#[test]
fn test_max_one() {
    assert_eq!(linear_congruential_generator(42, Some(1), None, None), 0);
}

#[test]
fn test_max_two() {
    assert_eq!(linear_congruential_generator(42, Some(2), None, None), 1);
}

#[test]
fn test_max_i32_max() {
    assert_eq!(
        linear_congruential_generator(42, Some(1000000), None, None),
        814273
    );
}

#[test]
fn test_multiplier_one() {
    assert_eq!(
        linear_congruential_generator(42, None, Some(1), None),
        1013904265
    );
}

#[test]
fn test_multiplier_i32_max() {
    assert_eq!(
        linear_congruential_generator(42, None, Some(100), None),
        1013908423
    );
}

#[test]
fn test_increment_zero() {
    assert_eq!(
        linear_congruential_generator(42, None, None, Some(0)),
        69910050
    );
}

#[test]
fn test_increment_one() {
    assert_eq!(
        linear_congruential_generator(42, None, None, Some(1)),
        69910051
    );
}

#[test]
fn test_increment_i32_max() {
    assert_eq!(
        linear_congruential_generator(42, None, None, Some(100)),
        69910150
    );
}

#[test]
fn test_all_custom() {
    assert_eq!(
        linear_congruential_generator(42, Some(100), Some(13), Some(17)),
        63
    );
}

#[test]
fn test_custom_max_multiplier() {
    assert_eq!(
        linear_congruential_generator(42, Some(100), Some(13), None),
        69
    );
}

#[test]
fn test_custom_max_increment() {
    assert_eq!(
        linear_congruential_generator(42, Some(100), None, Some(17)),
        67
    );
}

#[test]
fn test_custom_multiplier_increment() {
    assert_eq!(
        linear_congruential_generator(42, None, Some(13), Some(17)),
        563
    );
}

#[test]
fn test_max_zero() {
    assert!(std::panic::catch_unwind(|| {
        linear_congruential_generator(42, Some(0), None, None)
    })
    .is_err());
}

#[test]
fn test_max_minus_one() {
    linear_congruential_generator(42, Some(-1), None, None);
}

#[test]
fn test_max_i32_min() {
    linear_congruential_generator(42, Some(i32::MIN), None, None);
}

#[test]
fn test_multiplier_zero() {
    linear_congruential_generator(42, None, Some(0), None);
}

#[test]
fn test_multiplier_minus_one() {
    linear_congruential_generator(42, None, Some(-1), None);
}

#[test]
fn test_multiplier_i32_min() {
    assert!(std::panic::catch_unwind(|| {
        linear_congruential_generator(42, None, Some(i32::MIN), None)
    })
    .is_err());
}
