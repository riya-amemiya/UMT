use umtn_wasm::deg_to_rad;

#[test]
fn test_deg_to_rad() {
    assert_eq!(deg_to_rad(0.0), 0.0);
    assert_eq!(deg_to_rad(90.0), 1.5707963267948966);
    assert_eq!(deg_to_rad(180.0), 3.141592653589793);
    assert_eq!(deg_to_rad(270.0), 4.71238898038469);
    assert_eq!(deg_to_rad(360.0), 6.283185307179586);
}
