//! Array Type integration tests.
//!
//! Ported from TypeScript: arrayType.test.ts
//!
//! Tests the chunk array functionality which is the Rust equivalent
//! of the TypeScript ChunkArrayType.

use umt_rust::array::umt_chunk;

// ============================================================================
// Chunk Array Type Tests
// ============================================================================

#[test]
fn test_chunk_array_type_basic() {
    // TypeScript equivalent:
    // const _: ChunkArrayType<[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3> = [
    //     [1, 2, 3],
    //     [4, 5, 6],
    //     [7, 8, 9],
    //     [10],
    // ];

    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = umt_chunk(&input, 3);

    let expected: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_equal_chunks() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = umt_chunk(&input, 3);

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_single_element_chunks() {
    let input = vec![1, 2, 3, 4, 5];
    let result = umt_chunk(&input, 1);

    let expected = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_chunk_size_larger_than_array() {
    let input = vec![1, 2, 3];
    let result = umt_chunk(&input, 10);

    let expected = vec![vec![1, 2, 3]];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_empty_array() {
    let input: Vec<i32> = vec![];
    let result = umt_chunk(&input, 3);

    let expected: Vec<Vec<i32>> = vec![];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_with_strings() {
    let input = vec!["a", "b", "c", "d", "e", "f", "g"];
    let result = umt_chunk(&input, 2);

    let expected = vec![vec!["a", "b"], vec!["c", "d"], vec!["e", "f"], vec!["g"]];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_preserves_type() {
    // Test that chunking preserves the inner type
    #[derive(Debug, Clone, PartialEq)]
    struct CustomType {
        value: i32,
    }

    let input = vec![
        CustomType { value: 1 },
        CustomType { value: 2 },
        CustomType { value: 3 },
        CustomType { value: 4 },
        CustomType { value: 5 },
    ];

    let result = umt_chunk(&input, 2);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].len(), 2);
    assert_eq!(result[1].len(), 2);
    assert_eq!(result[2].len(), 1);

    assert_eq!(result[0][0].value, 1);
    assert_eq!(result[0][1].value, 2);
    assert_eq!(result[2][0].value, 5);
}

#[test]
fn test_chunk_array_type_chunk_size_two() {
    let input: Vec<i32> = (1..=10).collect();
    let result = umt_chunk(&input, 2);

    let expected = vec![
        vec![1, 2],
        vec![3, 4],
        vec![5, 6],
        vec![7, 8],
        vec![9, 10],
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_chunk_array_type_chunk_size_five() {
    let input: Vec<i32> = (1..=12).collect();
    let result = umt_chunk(&input, 5);

    let expected = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12]];

    assert_eq!(result, expected);
}
