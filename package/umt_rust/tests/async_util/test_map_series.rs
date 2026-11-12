use std::cell::RefCell;
use umt_rust::async_util::umt_map_series;

#[test]
fn test_map_series_processes_all_items_in_order() {
    let items = vec![1, 2, 3, 4, 5];
    let results = umt_map_series(&items, |n, _| Ok::<i32, String>(n * 10));
    assert_eq!(results, Ok(vec![10, 20, 30, 40, 50]));
}

#[test]
fn test_map_series_runs_strictly_sequentially() {
    let order = RefCell::new(Vec::new());
    let items = vec![1, 2, 3, 4];
    let results = umt_map_series(&items, |n, _| {
        order.borrow_mut().push(*n);
        Ok::<i32, String>(*n)
    });
    assert_eq!(results, Ok(vec![1, 2, 3, 4]));
    assert_eq!(*order.borrow(), vec![1, 2, 3, 4]);
}

#[test]
fn test_map_series_handles_empty_input() {
    let items: Vec<i32> = vec![];
    let results = umt_map_series(&items, |n, _| Ok::<i32, String>(n * 2));
    assert_eq!(results, Ok(vec![]));
}

#[test]
fn test_map_series_provides_index_to_the_function() {
    let items = vec!["a", "b", "c"];
    let results = umt_map_series(&items, |item, index| {
        Ok::<String, String>(format!("{}-{}", item, index))
    });
    assert_eq!(
        results,
        Ok(vec![
            "a-0".to_string(),
            "b-1".to_string(),
            "c-2".to_string()
        ])
    );
}

#[test]
fn test_map_series_stops_at_the_first_error() {
    let mut seen = Vec::new();
    let items = vec![1, 2, 3];
    let results = umt_map_series(&items, |n, _| {
        seen.push(*n);
        if *n == 2 {
            Err("fail".to_string())
        } else {
            Ok(*n)
        }
    });
    assert_eq!(results, Err("fail".to_string()));
    assert_eq!(seen, vec![1, 2]);
}

#[test]
fn test_map_series_handles_single_element() {
    let items = vec![7];
    let results = umt_map_series(&items, |n, _| Ok::<i32, String>(n + 1));
    assert_eq!(results, Ok(vec![8]));
}
