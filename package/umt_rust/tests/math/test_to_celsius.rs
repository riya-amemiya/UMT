use umt_rust::math::umt_to_celsius;

#[test]
fn test_to_celsius_freezing() {
    assert!((umt_to_celsius(273.15) - 0.0).abs() < 1e-10);
}

#[test]
fn test_to_celsius_300k() {
    assert!((umt_to_celsius(300.0) - 26.85).abs() < 1e-10);
}

#[test]
fn test_to_celsius_boiling() {
    assert!((umt_to_celsius(373.15) - 100.0).abs() < 1e-10);
}

#[test]
fn test_to_celsius_absolute_zero() {
    assert!((umt_to_celsius(0.0) - (-273.15)).abs() < 1e-10);
}

#[test]
fn test_to_celsius_32k() {
    assert!((umt_to_celsius(32.0) - (-241.15)).abs() < 1e-10);
}

#[test]
fn test_to_celsius_1000k() {
    assert!((umt_to_celsius(1000.0) - 726.85).abs() < 1e-10);
}

#[test]
fn test_to_celsius_negative_kelvin() {
    assert!((umt_to_celsius(-100.0) - (-373.15)).abs() < 1e-10);
}
