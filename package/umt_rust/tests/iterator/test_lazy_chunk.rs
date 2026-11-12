use std::cell::Cell;
use std::rc::Rc;
use umt_rust::iterator::lazy_chunk;

#[test]
fn test_groups_values_into_chunks() {
    let chunks: Vec<Vec<i32>> = lazy_chunk(vec![1, 2, 3, 4, 5, 6], 2).collect();
    assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

#[test]
fn test_yields_shorter_last_chunk() {
    let chunks: Vec<Vec<i32>> = lazy_chunk(vec![1, 2, 3, 4, 5], 2).collect();
    assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn test_one_chunk_when_size_exceeds_length() {
    let chunks: Vec<Vec<i32>> = lazy_chunk(vec![1, 2], 5).collect();
    assert_eq!(chunks, vec![vec![1, 2]]);
}

#[test]
fn test_empty_iterable() {
    let chunks: Vec<Vec<i32>> = lazy_chunk(Vec::<i32>::new(), 3).collect();
    assert!(chunks.is_empty());
}

#[test]
fn test_stays_exhausted() {
    let mut chunks = lazy_chunk(vec![1], 1);
    assert_eq!(chunks.next(), Some(vec![1]));
    assert_eq!(chunks.next(), None);
    assert_eq!(chunks.next(), None);
}

#[test]
fn test_works_with_infinite_iterables_lazily() {
    let pulled = Rc::new(Cell::new(0));
    let counter = pulled.clone();
    let source = (1..).map(move |n| {
        counter.set(counter.get() + 1);
        n
    });
    let mut chunks = lazy_chunk(source, 3);
    assert_eq!(chunks.next(), Some(vec![1, 2, 3]));
    assert_eq!(chunks.next(), Some(vec![4, 5, 6]));
    assert_eq!(pulled.get(), 6);
}
