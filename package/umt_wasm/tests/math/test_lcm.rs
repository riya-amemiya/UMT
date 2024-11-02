use umtn_wasm::lcm;

#[test]
fn test_lcm_of_2_and_3() {
    assert_eq!(lcm(2, 3), 6);
}

#[test]
fn test_lcm_of_6_and_8() {
    assert_eq!(lcm(6, 8), 24);
}

#[test]
fn test_lcm_of_21_and_6() {
    assert_eq!(lcm(21, 6), 42);
}

#[test]
fn test_lcm_of_0_and_10() {
    assert_eq!(lcm(0, 10), 0);
}

#[test]
fn test_lcm_of_7_and_5() {
    assert_eq!(lcm(7, 5), 35);
}

#[test]
fn test_lcm_of_15_and_20() {
    assert_eq!(lcm(15, 20), 60);
}

#[test]
fn test_lcm_of_1_and_1() {
    assert_eq!(lcm(1, 1), 1);
}
