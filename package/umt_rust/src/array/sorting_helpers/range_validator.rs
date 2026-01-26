/// Result of validating sort range indices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedSortRange {
    pub start_index: usize,
    pub end_index: usize,
    pub should_sort: bool,
}

/// Validates and adjusts the start and end indices for an operation on an array.
/// Ensures indices are within the bounds of the array.
///
/// # Arguments
///
/// * `array_len` - Length of the array
/// * `start_index` - The desired starting index
/// * `end_index` - The desired ending index
///
/// # Returns
///
/// A `ValidatedSortRange` containing the validated start index, end index,
/// and a boolean indicating if the operation should proceed on the range.
pub fn validate_range(
    array_len: usize,
    start_index: usize,
    end_index: usize,
) -> ValidatedSortRange {
    // Handle empty arrays early
    if array_len == 0 {
        return ValidatedSortRange {
            start_index: 0,
            end_index: 0,
            should_sort: false,
        };
    }

    let validated_start_index = start_index.min(array_len - 1);
    let validated_end_index = end_index.min(array_len - 1).max(validated_start_index);

    ValidatedSortRange {
        start_index: validated_start_index,
        end_index: validated_end_index,
        should_sort: validated_end_index >= validated_start_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_range_empty_array() {
        let result = validate_range(0, 0, 0);
        assert_eq!(result.start_index, 0);
        assert_eq!(result.end_index, 0);
        assert!(!result.should_sort);
    }

    #[test]
    fn test_validate_range_normal() {
        let result = validate_range(10, 2, 7);
        assert_eq!(result.start_index, 2);
        assert_eq!(result.end_index, 7);
        assert!(result.should_sort);
    }

    #[test]
    fn test_validate_range_exceeds_bounds() {
        let result = validate_range(5, 0, 10);
        assert_eq!(result.start_index, 0);
        assert_eq!(result.end_index, 4);
        assert!(result.should_sort);
    }

    #[test]
    fn test_validate_range_start_exceeds_bounds() {
        let result = validate_range(5, 10, 15);
        assert_eq!(result.start_index, 4);
        assert_eq!(result.end_index, 4);
        assert!(result.should_sort);
    }

    #[test]
    fn test_validate_range_single_element() {
        let result = validate_range(1, 0, 0);
        assert_eq!(result.start_index, 0);
        assert_eq!(result.end_index, 0);
        assert!(result.should_sort);
    }
}
