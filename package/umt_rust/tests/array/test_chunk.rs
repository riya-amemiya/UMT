use umt_rust::array::umt_chunk;

#[test]
fn test_chunk_splits_array() {
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let expected = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]];
    assert_eq!(umt_chunk(&input, 3), expected);
}

#[test]
fn test_chunk_empty_array() {
    let input: Vec<i32> = vec![];
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(umt_chunk(&input, 3), expected);
}

#[test]
fn test_chunk_larger_than_array() {
    let input = vec![0, 1, 2];
    let expected = vec![vec![0, 1, 2]];
    assert_eq!(umt_chunk(&input, 5), expected);
}

#[test]
fn test_chunk_size_one() {
    let input = vec![1, 2, 3, 4, 5];
    let expected = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
    assert_eq!(umt_chunk(&input, 1), expected);
}

#[test]
fn test_chunk_strings() {
    let input = vec!["a", "b", "c", "d", "e", "f", "g"];
    let expected = vec![vec!["a", "b", "c"], vec!["d", "e", "f"], vec!["g"]];
    assert_eq!(umt_chunk(&input, 3), expected);
}

#[test]
fn test_chunk_perfect_division() {
    let input = vec![1, 2, 3, 4, 5, 6];
    let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(umt_chunk(&input, 3), expected);
}

#[test]
fn test_chunk_equal_to_size() {
    let input = vec![1, 2, 3];
    let expected = vec![vec![1, 2, 3]];
    assert_eq!(umt_chunk(&input, 3), expected);
}

#[test]
fn test_chunk_single_element() {
    let input = vec![42];
    let expected = vec![vec![42]];
    assert_eq!(umt_chunk(&input, 1), expected);
}

#[test]
fn test_chunk_zero_size() {
    let input = vec![1, 2, 3];
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(umt_chunk(&input, 0), expected);
}

#[test]
fn test_chunk_does_not_mutate_input() {
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let _ = umt_chunk(&input, 3);
    assert_eq!(input, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_chunk_large_array() {
    let input: Vec<i32> = (0..10_000).collect();
    let output = umt_chunk(&input, 1000);
    assert_eq!(output.len(), 10);
    assert_eq!(output[0], (0..1000).collect::<Vec<i32>>());
    assert_eq!(output[9], (9000..10000).collect::<Vec<i32>>());
}
