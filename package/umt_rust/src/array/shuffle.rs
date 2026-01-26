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
