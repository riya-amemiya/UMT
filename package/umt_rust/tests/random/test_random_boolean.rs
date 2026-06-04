use umt_rust::random::{umt_random_boolean, umt_random_boolean_default};

#[test]
fn test_returns_a_boolean() {
    let value = umt_random_boolean_default();
    assert!(value == true || value == false);
}

#[test]
fn test_returns_true_when_probability_is_1() {
    for _ in 0..50 {
        assert_eq!(umt_random_boolean(1.0), true);
    }
}

#[test]
fn test_returns_false_when_probability_is_0() {
    for _ in 0..50 {
        assert_eq!(umt_random_boolean(0.0), false);
    }
}

#[test]
fn test_respects_probability_bias_roughly() {
    let mut true_count = 0;
    let trials = 2000;
    for _ in 0..trials {
        if umt_random_boolean(0.9) {
            true_count += 1;
        }
    }
    assert!(true_count as f64 / trials as f64 > 0.7);
}
