use umt_rust::math::umt_npr;

#[test]
fn test_npr() {
    assert_eq!(umt_npr(7, 3), 210);
    assert_eq!(umt_npr(5, 5), 120);
    assert_eq!(umt_npr(3, 5), 6);
    assert_eq!(umt_npr(5, 0), 1);
}
