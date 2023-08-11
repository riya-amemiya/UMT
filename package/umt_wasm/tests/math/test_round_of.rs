use umtn_wasm::round_of;

#[test]
fn test_round_of(){
    assert_eq!(round_of(1.23456789, 0), 1.0);
    assert_eq!(round_of(1.23456789, 1), 1.2);
    assert_eq!(round_of(1.23456789, 2), 1.23);
    assert_eq!(round_of(1.23456789, 3), 1.235);
    assert_eq!(round_of(1.23456789, 4), 1.2346);
}