use rand::Rng;

/// Randomly shuffles the elements of an array using the Fisher-Yates algorithm.
///
/// # Arguments
///
/// * `array` - Array to shuffle
///
/// # Returns
///
/// New vector with shuffled elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_shuffle;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let shuffled = umt_shuffle(&arr);
/// // shuffled contains the same elements in a different order
/// assert_eq!(shuffled.len(), arr.len());
/// ```
pub fn umt_shuffle<T: Clone>(array: &[T]) -> Vec<T> {
    let mut shuffled = array.to_vec();

    if shuffled.len() <= 1 {
        return shuffled;
    }

    let mut rng = rand::rng();

    for i in (1..shuffled.len()).rev() {
        let j = rng.random_range(0..=i);
        if i != j {
            shuffled.swap(i, j);
        }
    }

    shuffled
}

/// Shuffles a mutable array in place using the Fisher-Yates algorithm.
///
/// # Arguments
///
/// * `array` - Mutable array to shuffle
pub fn umt_shuffle_in_place<T>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let mut rng = rand::rng();

    for i in (1..array.len()).rev() {
        let j = rng.random_range(0..=i);
        if i != j {
            array.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_shuffle_maintains_length() {
        let array = vec![1, 2, 3, 4, 5];
        let shuffled = umt_shuffle(&array);
        assert_eq!(shuffled.len(), array.len());
    }

    #[test]
    fn test_shuffle_maintains_elements() {
        let array = vec![1, 2, 3, 4, 5];
        let shuffled = umt_shuffle(&array);

        let original_set: HashSet<_> = array.iter().collect();
        let shuffled_set: HashSet<_> = shuffled.iter().collect();
        assert_eq!(original_set, shuffled_set);
    }

    #[test]
    fn test_shuffle_empty_array() {
        let array: Vec<i32> = vec![];
        let shuffled = umt_shuffle(&array);
        assert_eq!(shuffled, Vec::<i32>::new());
    }

    #[test]
    fn test_shuffle_single_element() {
        let array = vec![1];
        let shuffled = umt_shuffle(&array);
        assert_eq!(shuffled, vec![1]);
    }

    #[test]
    fn test_shuffle_does_not_modify_original() {
        let array = vec![1, 2, 3, 4, 5];
        let original = array.clone();
        let _ = umt_shuffle(&array);
        assert_eq!(array, original);
    }

    #[test]
    fn test_shuffle_in_place() {
        let mut array = vec![1, 2, 3, 4, 5];
        let original_set: HashSet<_> = array.iter().cloned().collect();

        umt_shuffle_in_place(&mut array);

        let shuffled_set: HashSet<_> = array.iter().cloned().collect();
        assert_eq!(original_set, shuffled_set);
    }

    #[test]
    fn test_shuffle_with_duplicates() {
        let array = vec![1, 1, 2, 2, 3, 3];
        let shuffled = umt_shuffle(&array);

        // Count elements
        let count = |arr: &[i32], val: i32| arr.iter().filter(|&&x| x == val).count();

        assert_eq!(count(&shuffled, 1), 2);
        assert_eq!(count(&shuffled, 2), 2);
        assert_eq!(count(&shuffled, 3), 2);
    }

    #[test]
    fn test_shuffle_strings() {
        let array = vec!["a", "b", "c", "d", "e"];
        let shuffled = umt_shuffle(&array);

        let original_set: HashSet<_> = array.iter().collect();
        let shuffled_set: HashSet<_> = shuffled.iter().collect();
        assert_eq!(original_set, shuffled_set);
    }
}
