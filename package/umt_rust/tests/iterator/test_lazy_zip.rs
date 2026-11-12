use umt_rust::iterator::{lazy_zip, lazy_zip_many};

#[test]
fn test_zip_combines_values() {
    let pairs: Vec<(i32, char)> = lazy_zip(vec![1, 2, 3], vec!['a', 'b', 'c']).collect();
    assert_eq!(pairs, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn test_zip_stops_at_shortest() {
    let pairs: Vec<(i32, char)> = lazy_zip(vec![1, 2, 3], vec!['a']).collect();
    assert_eq!(pairs, vec![(1, 'a')]);
}

#[test]
fn test_zip_with_infinite_iterable() {
    let pairs: Vec<(u32, char)> = lazy_zip(0.., vec!['a', 'b']).collect();
    assert_eq!(pairs, vec![(0, 'a'), (1, 'b')]);
}

#[test]
fn test_zip_many_combines_values() {
    let rows: Vec<Vec<i32>> = lazy_zip_many(vec![vec![1, 2], vec![3, 4], vec![5, 6]]).collect();
    assert_eq!(rows, vec![vec![1, 3, 5], vec![2, 4, 6]]);
}

#[test]
fn test_zip_many_stops_at_shortest() {
    let rows: Vec<Vec<i32>> = lazy_zip_many(vec![vec![1, 2, 3], vec![4]]).collect();
    assert_eq!(rows, vec![vec![1, 4]]);
}

#[test]
fn test_zip_many_without_iterables() {
    let rows: Vec<Vec<i32>> = lazy_zip_many(Vec::<Vec<i32>>::new()).collect();
    assert!(rows.is_empty());
}

#[test]
fn test_zip_many_single_iterable() {
    let rows: Vec<Vec<i32>> = lazy_zip_many(vec![vec![1, 2]]).collect();
    assert_eq!(rows, vec![vec![1], vec![2]]);
}

#[test]
fn test_zip_many_stays_exhausted() {
    let mut rows = lazy_zip_many(vec![vec![1], vec![2, 3]]);
    assert_eq!(rows.next(), Some(vec![1, 2]));
    assert_eq!(rows.next(), None);
    assert_eq!(rows.next(), None);
}
