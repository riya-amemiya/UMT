use umt_rust::math::{umt_repeated_trial, Probability};

#[test]
fn test_repeated_trial_basic() {
    let (num, den) = umt_repeated_trial(4, 2, Probability { x: 1.0, y: 3.0 });
    assert_eq!(num, 8);
    assert_eq!(den, 27);
}

#[test]
fn test_repeated_trial_5_2() {
    let (num, den) = umt_repeated_trial(5, 2, Probability { x: 1.0, y: 2.0 });
    assert_eq!(num, 5);
    assert_eq!(den, 16);
}

#[test]
fn test_repeated_trial_3_2() {
    let (num, den) = umt_repeated_trial(3, 2, Probability { x: 1.0, y: 2.0 });
    assert_eq!(num, 3);
    assert_eq!(den, 8);
}

#[test]
fn test_repeated_trial_extreme_3_3() {
    let (num, den) = umt_repeated_trial(3, 3, Probability { x: 2.0, y: 3.0 });
    assert_eq!(num, 8);
    assert_eq!(den, 27);
}

#[test]
fn test_repeated_trial_4_4() {
    let (num, den) = umt_repeated_trial(4, 4, Probability { x: 1.0, y: 2.0 });
    assert_eq!(num, 1);
    assert_eq!(den, 16);
}

#[test]
fn test_repeated_trial_simple() {
    let (num, den) = umt_repeated_trial(2, 1, Probability { x: 1.0, y: 2.0 });
    assert_eq!(num, 1);
    assert_eq!(den, 2);
}

#[test]
fn test_repeated_trial_3_2_third() {
    let (num, den) = umt_repeated_trial(3, 2, Probability { x: 1.0, y: 3.0 });
    assert_eq!(num, 2);
    assert_eq!(den, 9);
}
