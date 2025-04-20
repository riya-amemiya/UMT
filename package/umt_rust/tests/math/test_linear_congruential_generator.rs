use umt_rust::math::umt_linear_congruential_generator;

#[test]
fn test_umt_linear_congruential_generator() {
    assert_eq!(
        umt_linear_congruential_generator(1, None, None, None),
        1015568748
    );
    assert_eq!(
        umt_linear_congruential_generator(10, None, None, None),
        1030549473
    );
    assert_eq!(
        umt_linear_congruential_generator(1, Some(100), Some(2), Some(3)),
        5
    );
}
