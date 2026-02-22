use rand::RngExt;
use umt_rust::array::umt_ultra_number_sort;

#[test]
fn test_sort_ascending_by_default() {
    let mut arr = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![1.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 9.0]);
}

#[test]
fn test_sort_descending() {
    let mut arr = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    umt_ultra_number_sort(&mut arr, false);
    assert_eq!(arr, vec![9.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 1.0]);
}

#[test]
fn test_empty_array() {
    let mut arr: Vec<f64> = vec![];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, Vec::<f64>::new());

    let mut arr2: Vec<f64> = vec![];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, Vec::<f64>::new());
}

#[test]
fn test_single_element() {
    let mut arr = vec![1.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![1.0]);

    let mut arr2 = vec![1.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![1.0]);
}

#[test]
fn test_two_elements_ascending() {
    let mut arr1 = vec![2.0, 1.0];
    umt_ultra_number_sort(&mut arr1, true);
    assert_eq!(arr1, vec![1.0, 2.0]);

    let mut arr2 = vec![1.0, 2.0];
    umt_ultra_number_sort(&mut arr2, true);
    assert_eq!(arr2, vec![1.0, 2.0]);
}

#[test]
fn test_two_elements_descending() {
    let mut arr1 = vec![2.0, 1.0];
    umt_ultra_number_sort(&mut arr1, false);
    assert_eq!(arr1, vec![2.0, 1.0]);

    let mut arr2 = vec![1.0, 2.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![2.0, 1.0]);
}

#[test]
fn test_three_elements_ascending() {
    let mut arr1 = vec![3.0, 1.0, 2.0];
    umt_ultra_number_sort(&mut arr1, true);
    assert_eq!(arr1, vec![1.0, 2.0, 3.0]);

    let mut arr2 = vec![1.0, 3.0, 2.0];
    umt_ultra_number_sort(&mut arr2, true);
    assert_eq!(arr2, vec![1.0, 2.0, 3.0]);

    let mut arr3 = vec![2.0, 1.0, 3.0];
    umt_ultra_number_sort(&mut arr3, true);
    assert_eq!(arr3, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_three_elements_descending() {
    let mut arr1 = vec![3.0, 1.0, 2.0];
    umt_ultra_number_sort(&mut arr1, false);
    assert_eq!(arr1, vec![3.0, 2.0, 1.0]);

    let mut arr2 = vec![1.0, 3.0, 2.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![3.0, 2.0, 1.0]);

    let mut arr3 = vec![2.0, 1.0, 3.0];
    umt_ultra_number_sort(&mut arr3, false);
    assert_eq!(arr3, vec![3.0, 2.0, 1.0]);
}

#[test]
fn test_duplicate_values() {
    let mut arr = vec![5.0, 2.0, 8.0, 2.0, 5.0, 8.0, 2.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![2.0, 2.0, 2.0, 5.0, 5.0, 8.0, 8.0]);

    let mut arr2 = vec![5.0, 2.0, 8.0, 2.0, 5.0, 8.0, 2.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![8.0, 8.0, 5.0, 5.0, 2.0, 2.0, 2.0]);
}

#[test]
fn test_negative_numbers() {
    let mut arr = vec![-3.0, 1.0, -4.0, 0.0, 5.0, -9.0, 2.0, -6.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![-9.0, -6.0, -4.0, -3.0, 0.0, 1.0, 2.0, 5.0]);

    let mut arr2 = vec![-3.0, 1.0, -4.0, 0.0, 5.0, -9.0, 2.0, -6.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![5.0, 2.0, 1.0, 0.0, -3.0, -4.0, -6.0, -9.0]);
}

#[test]
fn test_floating_point_numbers() {
    let mut arr = vec![3.14, 1.0, 4.5, 1.0, 5.9, 9.2, 2.6, 6.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![1.0, 1.0, 2.6, 3.14, 4.5, 5.9, 6.0, 9.2]);

    let mut arr2 = vec![3.14, 1.0, 4.5, 1.0, 5.9, 9.2, 2.6, 6.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![9.2, 6.0, 5.9, 4.5, 3.14, 2.6, 1.0, 1.0]);
}

#[test]
fn test_already_sorted() {
    let mut arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    let mut arr2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![5.0, 4.0, 3.0, 2.0, 1.0]);
}

#[test]
fn test_reverse_sorted() {
    let mut arr = vec![5.0, 4.0, 3.0, 2.0, 1.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    let mut arr2 = vec![5.0, 4.0, 3.0, 2.0, 1.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![5.0, 4.0, 3.0, 2.0, 1.0]);
}

#[test]
fn test_nan_values() {
    let mut arr = vec![3.0, f64::NAN, 1.0, 4.0, f64::NAN, 1.0, 5.0, 9.0, 2.0, 6.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(&arr[..8], &[1.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 9.0]);
    assert!(arr[8].is_nan());
    assert!(arr[9].is_nan());

    let mut arr2 = vec![3.0, f64::NAN, 1.0, 4.0, f64::NAN, 1.0, 5.0, 9.0, 2.0, 6.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(&arr2[..8], &[9.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 1.0]);
    assert!(arr2[8].is_nan());
    assert!(arr2[9].is_nan());
}

#[test]
fn test_only_nan_values() {
    let mut arr = vec![f64::NAN, f64::NAN, f64::NAN];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr.len(), 3);
    assert!(arr.iter().all(|x| x.is_nan()));
}

#[test]
fn test_infinity_values() {
    let mut arr = vec![f64::INFINITY, f64::NEG_INFINITY, 0.0, 100.0, -100.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(
        arr,
        vec![f64::NEG_INFINITY, -100.0, 0.0, 100.0, f64::INFINITY]
    );

    let mut arr2 = vec![f64::INFINITY, f64::NEG_INFINITY, 0.0, 100.0, -100.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(
        arr2,
        vec![f64::INFINITY, 100.0, 0.0, -100.0, f64::NEG_INFINITY]
    );
}

#[test]
fn test_all_same_elements() {
    let mut arr = vec![7.0, 7.0, 7.0, 7.0, 7.0];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![7.0, 7.0, 7.0, 7.0, 7.0]);

    let mut arr2 = vec![7.0, 7.0, 7.0, 7.0, 7.0];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![7.0, 7.0, 7.0, 7.0, 7.0]);
}

#[test]
fn test_counting_sort_small_integer_ranges() {
    use rand::rng;
    let mut rng = rng();

    let mut arr: Vec<f64> = (0..50)
        .map(|_| (rng.random_range(0..10) + 1) as f64)
        .collect();
    let mut expected = arr.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());

    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, expected);

    let mut arr2: Vec<f64> = (0..50)
        .map(|_| (rng.random_range(0..10) + 1) as f64)
        .collect();
    let mut expected2 = arr2.clone();
    expected2.sort_by(|a, b| b.partial_cmp(a).unwrap());

    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, expected2);
}

#[test]
fn test_radix_sort_larger_integer_arrays() {
    use rand::rng;
    let mut rng = rng();

    let mut arr: Vec<f64> = (0..200)
        .map(|_| (rng.random_range(0..1000) - 500) as f64)
        .collect();
    let mut expected = arr.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());

    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, expected);

    let mut arr2: Vec<f64> = (0..200)
        .map(|_| (rng.random_range(0..1000) - 500) as f64)
        .collect();
    let mut expected2 = arr2.clone();
    expected2.sort_by(|a, b| b.partial_cmp(a).unwrap());

    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, expected2);
}

#[test]
fn test_large_array_random_numbers() {
    use rand::rng;
    let mut rng = rng();

    let mut arr: Vec<f64> = (0..1000)
        .map(|_| rng.random_range(-1000.0..1000.0))
        .collect();
    let mut expected = arr.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());

    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, expected);

    let mut arr2: Vec<f64> = (0..1000)
        .map(|_| rng.random_range(-1000.0..1000.0))
        .collect();
    let mut expected2 = arr2.clone();
    expected2.sort_by(|a, b| b.partial_cmp(a).unwrap());

    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, expected2);
}

#[test]
fn test_radix_sort_only_positive() {
    use rand::rng;
    let mut rng = rng();

    let mut arr: Vec<f64> = (0..150)
        .map(|_| (rng.random_range(0..500) + 1) as f64)
        .collect();
    let mut expected = arr.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());

    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, expected);
}

#[test]
fn test_radix_sort_only_negative() {
    use rand::rng;
    let mut rng = rng();

    let mut arr: Vec<f64> = (0..150)
        .map(|_| -(rng.random_range(0..500) + 1) as f64)
        .collect();
    let mut expected = arr.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());

    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, expected);
}

#[test]
fn test_radix_sort_only_zeros() {
    let mut arr = vec![0.0; 101];
    umt_ultra_number_sort(&mut arr, true);
    assert_eq!(arr, vec![0.0; 101]);

    let mut arr2 = vec![0.0; 101];
    umt_ultra_number_sort(&mut arr2, false);
    assert_eq!(arr2, vec![0.0; 101]);
}

#[test]
fn test_radix_sort_with_single_positive() {
    let mut arr = vec![0.0; 101];
    arr.push(5.0);
    umt_ultra_number_sort(&mut arr, true);

    let mut expected = vec![0.0; 101];
    expected.push(5.0);
    assert_eq!(arr, expected);
}

#[test]
fn test_radix_sort_with_single_negative() {
    let mut arr = vec![0.0; 101];
    arr.push(-5.0);
    umt_ultra_number_sort(&mut arr, true);

    let mut expected = vec![-5.0];
    expected.extend(vec![0.0; 101]);
    assert_eq!(arr, expected);
}

#[test]
fn test_all_permutations_of_three_ascending() {
    let permutations = vec![
        vec![1.0, 2.0, 3.0],
        vec![1.0, 3.0, 2.0],
        vec![2.0, 1.0, 3.0],
        vec![2.0, 3.0, 1.0],
        vec![3.0, 1.0, 2.0],
        vec![3.0, 2.0, 1.0],
    ];
    let expected = vec![1.0, 2.0, 3.0];

    for perm in permutations {
        let mut arr = perm.clone();
        umt_ultra_number_sort(&mut arr, true);
        assert_eq!(arr, expected);
    }
}

#[test]
fn test_all_permutations_of_three_descending() {
    let permutations = vec![
        vec![1.0, 2.0, 3.0],
        vec![1.0, 3.0, 2.0],
        vec![2.0, 1.0, 3.0],
        vec![2.0, 3.0, 1.0],
        vec![3.0, 1.0, 2.0],
        vec![3.0, 2.0, 1.0],
    ];
    let expected = vec![3.0, 2.0, 1.0];

    for perm in permutations {
        let mut arr = perm.clone();
        umt_ultra_number_sort(&mut arr, false);
        assert_eq!(arr, expected);
    }
}
