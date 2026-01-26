/// Trait for checking if a value is "falsy" (similar to JavaScript's concept).
/// In Rust, we define falsy values as:
/// - 0 for numeric types
/// - false for bool
/// - empty string for String/&str
/// - NaN for floating point numbers
pub trait Falsy {
    fn is_falsy(&self) -> bool;
}

impl Falsy for i32 {
    #[inline]
    fn is_falsy(&self) -> bool {
        *self == 0
    }
}

impl Falsy for i64 {
    #[inline]
    fn is_falsy(&self) -> bool {
        *self == 0
    }
}

impl Falsy for f32 {
    #[inline]
    fn is_falsy(&self) -> bool {
        *self == 0.0 || self.is_nan()
    }
}

impl Falsy for f64 {
    #[inline]
    fn is_falsy(&self) -> bool {
        *self == 0.0 || self.is_nan()
    }
}

impl Falsy for bool {
    #[inline]
    fn is_falsy(&self) -> bool {
        !*self
    }
}

impl Falsy for String {
    #[inline]
    fn is_falsy(&self) -> bool {
        self.is_empty()
    }
}

impl Falsy for &str {
    #[inline]
    fn is_falsy(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Falsy for Option<T> {
    #[inline]
    fn is_falsy(&self) -> bool {
        self.is_none()
    }
}

/// Creates a vector with all falsy values removed.
/// Falsy values include: 0 (for numbers), false, empty string, NaN, and None.
///
/// # Arguments
///
/// * `array` - The array to compact
///
/// # Returns
///
/// Returns a new vector with filtered values
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_compact;
///
/// assert_eq!(umt_compact(&[0, 1, 2, 0, 3]), vec![1, 2, 3]);
/// ```
pub fn umt_compact<T: Clone + Falsy>(array: &[T]) -> Vec<T> {
    array.iter().filter(|x| !x.is_falsy()).cloned().collect()
}

/// Creates a vector with all None values removed from an Option vector.
///
/// # Arguments
///
/// * `array` - The array of Options to compact
///
/// # Returns
///
/// Returns a new vector with only Some values unwrapped
pub fn umt_compact_options<T: Clone>(array: &[Option<T>]) -> Vec<T> {
    array.iter().filter_map(|x| x.clone()).collect()
}
