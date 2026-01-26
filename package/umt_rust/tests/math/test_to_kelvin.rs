use umt_rust::math::umt_to_kelvin;

#[test]
fn test_to_kelvin_zero_celsius() {
    assert!((umt_to_kelvin(0.0) - 273.15).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_room_temp() {
    assert!((umt_to_kelvin(26.85) - 300.0).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_boiling() {
    assert!((umt_to_kelvin(100.0) - 373.15).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_negative_40() {
    assert!((umt_to_kelvin(-40.0) - 233.15).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_absolute_zero() {
    assert!((umt_to_kelvin(-273.15) - 0.0).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_1000_celsius() {
    assert!((umt_to_kelvin(1000.0) - 1273.15).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_negative_300() {
    assert!((umt_to_kelvin(-300.0) - (-26.85)).abs() < 1e-10);
}

#[test]
fn test_to_kelvin_decimal() {
    assert!((umt_to_kelvin(25.5) - 298.65).abs() < 1e-10);
}
