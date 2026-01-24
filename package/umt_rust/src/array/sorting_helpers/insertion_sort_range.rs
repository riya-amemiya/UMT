/// Performs insertion sort on a range of the array in-place.
///
/// # Arguments
///
/// * `array` - The array to sort
/// * `compare` - Comparison function that returns negative if a < b, zero if a == b, positive if a > b
/// * `start` - Starting index (inclusive)
/// * `end` - Ending index (inclusive)
///
/// # Type Parameters
///
/// * `T` - The type of elements in the array
/// * `F` - The comparison function type
pub fn insertion_sort_range<T, F>(array: &mut [T], compare: &F, start: usize, end: usize)
where
    F: Fn(&T, &T) -> i32,
{
    if start >= end || start >= array.len() {
        return;
    }

    let actual_end = end.min(array.len() - 1);

    for i in (start + 1)..=actual_end {
        let mut j = i;
        while j > start && compare(&array[j - 1], &array[j]) > 0 {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
        if a > b {
            1
        } else if a < b {
            -1
        } else {
            0
        }
    }

    #[test]
    fn test_insertion_sort_range_full() {
        let mut arr = vec![5, 2, 8, 1, 9];
        insertion_sort_range(&mut arr, &default_compare, 0, 4);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_insertion_sort_range_partial() {
        let mut arr = vec![5, 2, 8, 1, 9];
        insertion_sort_range(&mut arr, &default_compare, 1, 3);
        assert_eq!(arr, vec![5, 1, 2, 8, 9]);
    }

    #[test]
    fn test_insertion_sort_range_empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort_range(&mut arr, &default_compare, 0, 0);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_insertion_sort_range_single() {
        let mut arr = vec![42];
        insertion_sort_range(&mut arr, &default_compare, 0, 0);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_insertion_sort_range_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort_range(&mut arr, &default_compare, 0, 4);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_range_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort_range(&mut arr, &default_compare, 0, 4);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_range_strings() {
        let mut arr = vec!["banana", "apple", "cherry"];
        insertion_sort_range(&mut arr, &default_compare, 0, 2);
        assert_eq!(arr, vec!["apple", "banana", "cherry"]);
    }
}
