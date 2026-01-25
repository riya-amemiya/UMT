//! Integration tests for Array utility functions
//!
//! Tests the interaction between `drop` and `first` functions to ensure they work correctly
//! when used together in various scenarios:
//! - Dropping elements from the start (left) and getting the first element
//! - Dropping elements from the end (right) and getting the first element
//! - Edge cases with different array sizes and drop amounts

use umt_rust::array::{umt_drop, umt_drop_left, umt_drop_right, umt_first, DropDirection};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_drop_n_elements_from_the_array_and_return_the_first_element() {
        // first(drop([1, 2, 3, 4, 5])) -> 2
        let dropped = umt_drop_left(&[1, 2, 3, 4, 5], 1);
        assert_eq!(umt_first(&dropped), Some(&2));

        // first(drop([1, 2, 3, 4, 5], 2)) -> 3
        let dropped = umt_drop_left(&[1, 2, 3, 4, 5], 2);
        assert_eq!(umt_first(&dropped), Some(&3));

        // first(drop([1, 2, 3, 4, 5], 3)) -> 4
        let dropped = umt_drop_left(&[1, 2, 3, 4, 5], 3);
        assert_eq!(umt_first(&dropped), Some(&4));

        // first(drop([1, 2, 3, 4, 5], 4)) -> 5
        let dropped = umt_drop_left(&[1, 2, 3, 4, 5], 4);
        assert_eq!(umt_first(&dropped), Some(&5));

        // first(drop([1, 2, 3, 4, 5], 2, "right")) -> 1
        let dropped = umt_drop_right(&[1, 2, 3, 4, 5], 2);
        assert_eq!(umt_first(&dropped), Some(&1));

        // first(drop([1, 2, 3, 4, 5], 3, "left")) -> 4
        let dropped = umt_drop(&[1, 2, 3, 4, 5], 3, DropDirection::Left);
        assert_eq!(umt_first(&dropped), Some(&4));
    }

    #[test]
    fn should_handle_edge_cases_with_empty_arrays_and_large_drop_amounts() {
        // first(drop([])) -> None
        let dropped = umt_drop_left::<i32>(&[], 1);
        assert_eq!(umt_first(&dropped), None);

        // first(drop([1], 1)) -> None
        let dropped = umt_drop_left(&[1], 1);
        assert_eq!(umt_first(&dropped), None);

        // first(drop([1, 2], 5)) -> None
        let dropped = umt_drop_left(&[1, 2], 5);
        assert_eq!(umt_first(&dropped), None);

        // first(drop([1, 2, 3], 0)) -> 1
        let dropped = umt_drop_left(&[1, 2, 3], 0);
        assert_eq!(umt_first(&dropped), Some(&1));
    }

    #[test]
    fn should_handle_arrays_with_different_data_types() {
        // first(drop(["a", "b", "c"], 1)) -> "b"
        let dropped = umt_drop_left(&["a", "b", "c"], 1);
        assert_eq!(umt_first(&dropped), Some(&"b"));

        // first(drop([true, false], 1)) -> false
        let dropped = umt_drop_left(&[true, false], 1);
        assert_eq!(umt_first(&dropped), Some(&false));

        // first(drop([None, None, Some(1)], 2)) -> Some(1)
        let dropped = umt_drop_left(&[None, None, Some(1)], 2);
        assert_eq!(umt_first(&dropped), Some(&Some(1)));
    }
}
