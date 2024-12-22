use umt_rust::math::umt_npr;

#[test]
fn test_npr() {
    assert_eq!(umt_npr(7, 3), 210);
}
