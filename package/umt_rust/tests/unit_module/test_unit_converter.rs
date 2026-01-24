use umt_rust::unit::{umt_unit_converter, UmtUnitConverter};

/// Helper function to create a length converter for testing
fn create_length_converter() -> UmtUnitConverter<&'static str> {
    umt_unit_converter(vec![
        ("meters", 1.0),
        ("kilometers", 0.001),
        ("centimeters", 100.0),
        ("millimeters", 1000.0),
    ])
}

#[test]
fn test_converts_between_base_unit_and_larger_units() {
    let converter = create_length_converter();

    let result1 = converter.convert(1000.0, &"meters", &"kilometers").unwrap();
    assert!(
        (result1 - 1.0).abs() < 1e-10,
        "Expected 1.0, got {}",
        result1
    );

    let result2 = converter.convert(1.0, &"kilometers", &"meters").unwrap();
    assert!(
        (result2 - 1000.0).abs() < 1e-10,
        "Expected 1000.0, got {}",
        result2
    );
}

#[test]
fn test_converts_between_base_unit_and_smaller_units() {
    let converter = create_length_converter();

    let result1 = converter.convert(1.0, &"meters", &"centimeters").unwrap();
    assert!(
        (result1 - 100.0).abs() < 1e-10,
        "Expected 100.0, got {}",
        result1
    );

    let result2 = converter.convert(100.0, &"centimeters", &"meters").unwrap();
    assert!(
        (result2 - 1.0).abs() < 1e-10,
        "Expected 1.0, got {}",
        result2
    );
}

#[test]
fn test_converts_between_different_small_units() {
    let converter = create_length_converter();

    let result1 = converter
        .convert(100.0, &"centimeters", &"millimeters")
        .unwrap();
    assert!(
        (result1 - 1000.0).abs() < 1e-10,
        "Expected 1000.0, got {}",
        result1
    );

    let result2 = converter
        .convert(1000.0, &"millimeters", &"centimeters")
        .unwrap();
    assert!(
        (result2 - 100.0).abs() < 1e-10,
        "Expected 100.0, got {}",
        result2
    );
}

#[test]
fn test_handles_zero_values_correctly() {
    let converter = create_length_converter();

    let result1 = converter.convert(0.0, &"meters", &"kilometers").unwrap();
    assert_eq!(result1, 0.0);

    let result2 = converter
        .convert(0.0, &"millimeters", &"centimeters")
        .unwrap();
    assert_eq!(result2, 0.0);
}

#[test]
fn test_returns_same_value_when_converting_to_same_unit() {
    let converter = create_length_converter();

    let result1 = converter.convert(5.0, &"meters", &"meters").unwrap();
    assert_eq!(result1, 5.0);

    let result2 = converter
        .convert(100.0, &"centimeters", &"centimeters")
        .unwrap();
    assert_eq!(result2, 100.0);
}

#[test]
fn test_maintains_precision_in_round_trip_conversions() {
    let converter = create_length_converter();
    let original = 5.0;

    // Round trip: meters -> kilometers -> meters
    let km_value = converter
        .convert(original, &"meters", &"kilometers")
        .unwrap();
    let round_trip1 = converter
        .convert(km_value, &"kilometers", &"meters")
        .unwrap();
    assert!(
        (round_trip1 - original).abs() < 1e-10,
        "Round trip meters->kilometers->meters failed: {} != {}",
        round_trip1,
        original
    );

    // Round trip: centimeters -> millimeters -> centimeters
    let mm_value = converter
        .convert(original, &"centimeters", &"millimeters")
        .unwrap();
    let round_trip2 = converter
        .convert(mm_value, &"millimeters", &"centimeters")
        .unwrap();
    assert!(
        (round_trip2 - original).abs() < 1e-10,
        "Round trip centimeters->millimeters->centimeters failed: {} != {}",
        round_trip2,
        original
    );
}

#[test]
fn test_returns_none_for_unknown_units() {
    let converter = create_length_converter();

    assert!(
        converter.convert(100.0, &"unknown", &"meters").is_none(),
        "Should return None for unknown 'from' unit"
    );
    assert!(
        converter.convert(100.0, &"meters", &"unknown").is_none(),
        "Should return None for unknown 'to' unit"
    );
    assert!(
        converter
            .convert(100.0, &"unknown1", &"unknown2")
            .is_none(),
        "Should return None when both units are unknown"
    );
}

#[test]
fn test_has_unit() {
    let converter = create_length_converter();

    assert!(converter.has_unit(&"meters"));
    assert!(converter.has_unit(&"kilometers"));
    assert!(converter.has_unit(&"centimeters"));
    assert!(converter.has_unit(&"millimeters"));
    assert!(!converter.has_unit(&"unknown"));
    assert!(!converter.has_unit(&"feet"));
}

#[test]
fn test_unit_count() {
    let converter = create_length_converter();
    assert_eq!(converter.unit_count(), 4);

    let small_converter = umt_unit_converter(vec![("meters", 1.0), ("feet", 3.28084)]);
    assert_eq!(small_converter.unit_count(), 2);
}

#[test]
fn test_get_ratio() {
    let converter = create_length_converter();

    assert_eq!(converter.get_ratio(&"meters"), Some(&1.0));
    assert_eq!(converter.get_ratio(&"kilometers"), Some(&0.001));
    assert_eq!(converter.get_ratio(&"centimeters"), Some(&100.0));
    assert_eq!(converter.get_ratio(&"millimeters"), Some(&1000.0));
    assert_eq!(converter.get_ratio(&"unknown"), None);
}

#[test]
fn test_umt_unit_converter_function() {
    let converter = umt_unit_converter(vec![("meters", 1.0), ("feet", 3.28084)]);

    let result = converter.convert(1.0, &"meters", &"feet").unwrap();
    assert!(
        (result - 3.28084).abs() < 1e-10,
        "Expected 3.28084, got {}",
        result
    );
}

#[test]
fn test_with_string_keys() {
    let converter = UmtUnitConverter::new(vec![
        (String::from("meters"), 1.0),
        (String::from("kilometers"), 0.001),
        (String::from("centimeters"), 100.0),
    ]);

    let result = converter
        .convert(
            1000.0,
            &String::from("meters"),
            &String::from("kilometers"),
        )
        .unwrap();
    assert!((result - 1.0).abs() < 1e-10, "Expected 1.0, got {}", result);
}

#[test]
fn test_temperature_converter() {
    // Temperature conversion is different - not a simple ratio
    // But we can still use the converter for linear conversions
    // This tests that the converter works with different unit systems
    let weight_converter = umt_unit_converter(vec![
        ("grams", 1.0),
        ("kilograms", 0.001),
        ("milligrams", 1000.0),
        ("pounds", 0.00220462),
    ]);

    let result = weight_converter.convert(1.0, &"kilograms", &"grams").unwrap();
    assert!(
        (result - 1000.0).abs() < 1e-10,
        "Expected 1000.0 grams, got {}",
        result
    );

    let pounds = weight_converter.convert(1.0, &"kilograms", &"pounds").unwrap();
    assert!(
        (pounds - 2.20462).abs() < 1e-5,
        "Expected ~2.20462 pounds, got {}",
        pounds
    );
}

#[test]
fn test_negative_values() {
    let converter = create_length_converter();

    let result = converter.convert(-5.0, &"meters", &"centimeters").unwrap();
    assert!(
        (result - (-500.0)).abs() < 1e-10,
        "Expected -500.0, got {}",
        result
    );
}

#[test]
fn test_large_values() {
    let converter = create_length_converter();

    let result = converter
        .convert(1_000_000.0, &"meters", &"kilometers")
        .unwrap();
    assert!(
        (result - 1000.0).abs() < 1e-10,
        "Expected 1000.0, got {}",
        result
    );
}

#[test]
fn test_small_values() {
    let converter = create_length_converter();

    let result = converter.convert(0.001, &"meters", &"millimeters").unwrap();
    assert!((result - 1.0).abs() < 1e-10, "Expected 1.0, got {}", result);
}

#[test]
fn test_time_converter() {
    let time_converter = umt_unit_converter(vec![
        ("seconds", 1.0),
        ("minutes", 1.0 / 60.0),
        ("hours", 1.0 / 3600.0),
        ("milliseconds", 1000.0),
    ]);

    let result = time_converter.convert(3600.0, &"seconds", &"hours").unwrap();
    assert!((result - 1.0).abs() < 1e-10, "Expected 1.0, got {}", result);

    let result2 = time_converter.convert(1.0, &"hours", &"minutes").unwrap();
    assert!(
        (result2 - 60.0).abs() < 1e-10,
        "Expected 60.0, got {}",
        result2
    );
}
