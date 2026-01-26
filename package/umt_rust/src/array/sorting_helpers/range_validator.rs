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
