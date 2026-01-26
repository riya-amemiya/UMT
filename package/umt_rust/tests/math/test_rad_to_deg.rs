use std::f64::consts::PI;
use umt_rust::math::umt_rad_to_deg;

#[test]
fn test_rad_to_deg_zero() {
    assert_eq!(umt_rad_to_deg(0.0), 0.0);
}

#[test]
fn test_rad_to_deg_half_pi() {
    let result = umt_rad_to_deg(PI / 2.0);
    assert!((result - 90.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_pi() {
    let result = umt_rad_to_deg(PI);
    assert!((result - 180.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_three_halves_pi() {
    let result = umt_rad_to_deg(3.0 * PI / 2.0);
    assert!((result - 270.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_two_pi() {
    let result = umt_rad_to_deg(2.0 * PI);
    assert!((result - 360.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_negative_half_pi() {
    let result = umt_rad_to_deg(-PI / 2.0);
    assert!((result - (-90.0)).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_negative_pi() {
    let result = umt_rad_to_deg(-PI);
    assert!((result - (-180.0)).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_negative_three_halves_pi() {
    let result = umt_rad_to_deg(-3.0 * PI / 2.0);
    assert!((result - (-270.0)).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_negative_two_pi() {
    let result = umt_rad_to_deg(-2.0 * PI);
    assert!((result - (-360.0)).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_quarter_pi() {
    let result = umt_rad_to_deg(PI / 4.0);
    assert!((result - 45.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_custom_angle() {
    let result = umt_rad_to_deg(PI * 60.7 / 180.0);
    assert!((result - 60.7).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_over_two_pi() {
    let result = umt_rad_to_deg(5.0 * PI / 2.0);
    assert!((result - 450.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_four_pi() {
    let result = umt_rad_to_deg(4.0 * PI);
    assert!((result - 720.0).abs() < 1e-10);
}

#[test]
fn test_rad_to_deg_nan() {
    assert!(umt_rad_to_deg(f64::NAN).is_nan());
}

#[test]
fn test_rad_to_deg_infinity() {
    assert_eq!(umt_rad_to_deg(f64::INFINITY), f64::INFINITY);
    assert_eq!(umt_rad_to_deg(f64::NEG_INFINITY), f64::NEG_INFINITY);
}

use umt_rust::math::*;

#[test]
fn test_rad_to_deg_negative() {
    let degrees = umt_rad_to_deg(-PI);
    assert!((degrees - (-180.0)).abs() < 1e-10);
}
