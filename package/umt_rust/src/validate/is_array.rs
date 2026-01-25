//! Array type checking utility

/// Determines if the value is a Vec (array-like)
///
/// In Rust, we use Vec<T> as the array type. This function always returns true
/// for Vec inputs due to Rust's type system.
///
/// # Arguments
/// * `_array` - The Vec to check
///
/// # Returns
/// Always returns true since the input is typed as Vec
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_array;
///
/// let arr = vec![1, 2, 3];
/// assert!(umt_is_array(&arr));
/// ```
#[inline]
pub fn umt_is_array<T>(_array: &Vec<T>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_array() {
        assert!(umt_is_array(&vec![1, 2, 3]));
        assert!(umt_is_array(&vec!["a", "b"]));
        assert!(umt_is_array(&Vec::<i32>::new()));
    }
}
