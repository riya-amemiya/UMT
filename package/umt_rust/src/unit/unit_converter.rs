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
