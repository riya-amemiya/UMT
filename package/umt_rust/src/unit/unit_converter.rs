use std::collections::HashMap;
use std::hash::Hash;

/// A unit converter that can convert between different units using base unit ratios.
///
/// This struct stores conversion ratios to a base unit and provides a method
/// to convert values between any two units in the system.
///
/// # Type Parameters
///
/// * `T` - The type used to identify units (typically String, &str, or an enum)
///
/// # Example
///
/// ```
/// use umt_rust::unit::UmtUnitConverter;
///
/// let converter = UmtUnitConverter::new(vec![
///     ("meters", 1.0),       // base unit
///     ("kilometers", 0.001),
///     ("centimeters", 100.0),
///     ("millimeters", 1000.0),
/// ]);
///
/// // Convert 1000 meters to kilometers
/// let result = converter.convert(1000.0, &"meters", &"kilometers");
/// assert!((result.unwrap() - 1.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone)]
pub struct UmtUnitConverter<T>
where
    T: Eq + Hash,
{
    ratios: HashMap<T, f64>,
}

impl<T> UmtUnitConverter<T>
where
    T: Eq + Hash,
{
    /// Creates a new UmtUnitConverter with the given unit ratios.
    ///
    /// # Arguments
    ///
    /// * `ratios` - A vector of tuples containing unit identifiers and their ratios to the base unit
    ///
    /// # Returns
    ///
    /// A new UmtUnitConverter instance
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::unit::UmtUnitConverter;
    ///
    /// let converter = UmtUnitConverter::new(vec![
    ///     ("meters", 1.0),
    ///     ("kilometers", 0.001),
    /// ]);
    /// ```
    pub fn new(ratios: Vec<(T, f64)>) -> Self {
        UmtUnitConverter {
            ratios: ratios.into_iter().collect(),
        }
    }

    /// Converts a value from one unit to another.
    ///
    /// The conversion formula is: `(value / from_ratio) * to_ratio`
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to convert
    /// * `from` - The source unit identifier
    /// * `to` - The target unit identifier
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The converted value if both units are found
    /// * `None` - If either the source or target unit is not found
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::unit::UmtUnitConverter;
    ///
    /// let converter = UmtUnitConverter::new(vec![
    ///     ("meters", 1.0),
    ///     ("kilometers", 0.001),
    ///     ("centimeters", 100.0),
    /// ]);
    ///
    /// // Convert 5 kilometers to meters
    /// let result = converter.convert(5.0, &"kilometers", &"meters");
    /// assert!((result.unwrap() - 5000.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn convert(&self, value: f64, from: &T, to: &T) -> Option<f64> {
        let from_ratio = self.ratios.get(from)?;
        let to_ratio = self.ratios.get(to)?;

        // Avoid division by zero
        if *from_ratio == 0.0 {
            return Some(f64::NAN);
        }

        Some((value / from_ratio) * to_ratio)
    }

    /// Checks if a unit exists in this converter.
    ///
    /// # Arguments
    ///
    /// * `unit` - The unit identifier to check
    ///
    /// # Returns
    ///
    /// `true` if the unit exists, `false` otherwise
    #[inline]
    pub fn has_unit(&self, unit: &T) -> bool {
        self.ratios.contains_key(unit)
    }

    /// Returns the number of units in this converter.
    #[inline]
    pub fn unit_count(&self) -> usize {
        self.ratios.len()
    }

    /// Returns the ratio for a specific unit.
    ///
    /// # Arguments
    ///
    /// * `unit` - The unit identifier
    ///
    /// # Returns
    ///
    /// * `Some(&f64)` - The ratio if the unit exists
    /// * `None` - If the unit is not found
    #[inline]
    pub fn get_ratio(&self, unit: &T) -> Option<&f64> {
        self.ratios.get(unit)
    }
}

/// Creates a unit converter with the given ratios.
///
/// This is a convenience function that wraps `UmtUnitConverter::new()`.
///
/// # Arguments
///
/// * `ratios` - A vector of tuples containing unit identifiers and their ratios to the base unit
///
/// # Returns
///
/// A new UmtUnitConverter instance
///
/// # Example
///
/// ```
/// use umt_rust::unit::umt_unit_converter;
///
/// let converter = umt_unit_converter(vec![
///     ("meters", 1.0),
///     ("kilometers", 0.001),
///     ("centimeters", 100.0),
/// ]);
///
/// let result = converter.convert(100.0, &"centimeters", &"meters");
/// assert!((result.unwrap() - 1.0).abs() < 1e-10);
/// ```
pub fn umt_unit_converter<T>(ratios: Vec<(T, f64)>) -> UmtUnitConverter<T>
where
    T: Eq + Hash,
{
    UmtUnitConverter::new(ratios)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_length_converter() -> UmtUnitConverter<&'static str> {
        UmtUnitConverter::new(vec![
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
        assert!((result1 - 1.0).abs() < 1e-10);

        let result2 = converter.convert(1.0, &"kilometers", &"meters").unwrap();
        assert!((result2 - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_converts_between_base_unit_and_smaller_units() {
        let converter = create_length_converter();

        let result1 = converter.convert(1.0, &"meters", &"centimeters").unwrap();
        assert!((result1 - 100.0).abs() < 1e-10);

        let result2 = converter.convert(100.0, &"centimeters", &"meters").unwrap();
        assert!((result2 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_converts_between_different_small_units() {
        let converter = create_length_converter();

        let result1 = converter
            .convert(100.0, &"centimeters", &"millimeters")
            .unwrap();
        assert!((result1 - 1000.0).abs() < 1e-10);

        let result2 = converter
            .convert(1000.0, &"millimeters", &"centimeters")
            .unwrap();
        assert!((result2 - 100.0).abs() < 1e-10);
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
        assert!((round_trip1 - original).abs() < 1e-10);

        // Round trip: centimeters -> millimeters -> centimeters
        let mm_value = converter
            .convert(original, &"centimeters", &"millimeters")
            .unwrap();
        let round_trip2 = converter
            .convert(mm_value, &"millimeters", &"centimeters")
            .unwrap();
        assert!((round_trip2 - original).abs() < 1e-10);
    }

    #[test]
    fn test_returns_none_for_unknown_units() {
        let converter = create_length_converter();

        assert!(converter.convert(100.0, &"unknown", &"meters").is_none());
        assert!(converter.convert(100.0, &"meters", &"unknown").is_none());
        assert!(converter.convert(100.0, &"unknown1", &"unknown2").is_none());
    }

    #[test]
    fn test_has_unit() {
        let converter = create_length_converter();

        assert!(converter.has_unit(&"meters"));
        assert!(converter.has_unit(&"kilometers"));
        assert!(!converter.has_unit(&"unknown"));
    }

    #[test]
    fn test_unit_count() {
        let converter = create_length_converter();
        assert_eq!(converter.unit_count(), 4);
    }

    #[test]
    fn test_get_ratio() {
        let converter = create_length_converter();

        assert_eq!(converter.get_ratio(&"meters"), Some(&1.0));
        assert_eq!(converter.get_ratio(&"kilometers"), Some(&0.001));
        assert_eq!(converter.get_ratio(&"unknown"), None);
    }

    #[test]
    fn test_umt_unit_converter_function() {
        let converter = umt_unit_converter(vec![("meters", 1.0), ("feet", 3.28084)]);

        let result = converter.convert(1.0, &"meters", &"feet").unwrap();
        assert!((result - 3.28084).abs() < 1e-10);
    }

    #[test]
    fn test_with_string_keys() {
        let converter = UmtUnitConverter::new(vec![
            (String::from("meters"), 1.0),
            (String::from("kilometers"), 0.001),
        ]);

        let result = converter
            .convert(1000.0, &String::from("meters"), &String::from("kilometers"))
            .unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }
}
