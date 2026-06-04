use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use umt_rust::async_util::umt_parallel;

#[test]
fn test_parallel_processes_all_items_in_order() {
    let items = vec![1, 2, 3, 4, 5];
    let results = umt_parallel(2, &items, |n, _| Ok::<i32, String>(n * 10));
    assert_eq!(results, Ok(vec![10, 20, 30, 40, 50]));
}

#[test]
fn test_parallel_respects_the_concurrency_limit() {
    let running = AtomicUsize::new(0);
    let max_running = AtomicUsize::new(0);
    let items = vec![1, 2, 3, 4, 5, 6];
    let _ = umt_parallel(3, &items, |n, _| {
        let current = running.fetch_add(1, Ordering::SeqCst) + 1;
        max_running.fetch_max(current, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(10));
        running.fetch_sub(1, Ordering::SeqCst);
        Ok::<i32, String>(*n)
    });
    assert!(max_running.load(Ordering::SeqCst) <= 3);
}

#[test]
fn test_parallel_handles_empty_input() {
    let items: Vec<i32> = vec![];
    let results = umt_parallel(2, &items, |n, _| Ok::<i32, String>(n * 2));
    assert_eq!(results, Ok(vec![]));
}

#[test]
fn test_parallel_handles_limit_larger_than_length() {
    let items = vec![1, 2];
    let results = umt_parallel(10, &items, |n, _| Ok::<i32, String>(n + 1));
    assert_eq!(results, Ok(vec![2, 3]));
}

#[test]
fn test_parallel_rejects_when_any_task_fails() {
    let items = vec![1, 2, 3];
    let results = umt_parallel(2, &items, |n, _| {
        if *n == 2 {
            Err("fail".to_string())
        } else {
            Ok(*n)
        }
    });
    assert_eq!(results, Err("fail".to_string()));
}

#[test]
fn test_parallel_provides_index_to_the_function() {
    let items = vec!["a", "b", "c"];
    let results = umt_parallel(2, &items, |item, index| {
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
fn test_parallel_handles_limit_of_one_sequentially() {
    let order = Mutex::new(Vec::new());
    let items = vec![1, 2, 3];
    let _ = umt_parallel(1, &items, |n, _| {
        order.lock().unwrap().push(*n);
        Ok::<i32, String>(*n)
    });
    assert_eq!(*order.lock().unwrap(), vec![1, 2, 3]);
}
