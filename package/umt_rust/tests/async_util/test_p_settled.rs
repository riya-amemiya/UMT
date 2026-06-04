use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use umt_rust::async_util::{SettledResult, SettledTask, umt_p_settled};

#[test]
fn test_p_settled_mixed_fulfilled_and_rejected() {
    let tasks: Vec<SettledTask<i32, String>> =
        vec![Box::new(|| Ok(1)), Box::new(|| Err("nope".to_string()))];
    let results = umt_p_settled(tasks, None);
    assert_eq!(results[0], SettledResult::Fulfilled { value: 1 });
    assert!(matches!(results[1], SettledResult::Rejected { .. }));
}

#[test]
fn test_p_settled_empty_input() {
    let tasks: Vec<SettledTask<i32, String>> = vec![];
    let results = umt_p_settled(tasks, None);
    assert!(results.is_empty());
}

#[test]
fn test_p_settled_runs_every_task() {
    let started = AtomicUsize::new(0);
    let tasks: Vec<SettledTask<i32, String>> = vec![
        Box::new(|| {
            started.fetch_add(1, Ordering::SeqCst);
            Ok(1)
        }),
        Box::new(|| {
            started.fetch_add(1, Ordering::SeqCst);
            Ok(2)
        }),
    ];
    let results = umt_p_settled(tasks, Some(1));
    assert_eq!(started.load(Ordering::SeqCst), 2);
    let values: Vec<i32> = results
        .into_iter()
        .map(|r| match r {
            SettledResult::Fulfilled { value } => value,
            SettledResult::Rejected { .. } => -1,
        })
        .collect();
    assert_eq!(values, vec![1, 2]);
}

#[test]
fn test_p_settled_respects_concurrency_limit() {
    let active = AtomicUsize::new(0);
    let peak = AtomicUsize::new(0);
    let tasks: Vec<SettledTask<usize, String>> = (0..6)
        .map(|index| {
            let active = &active;
            let peak = &peak;
            Box::new(move || -> Result<usize, String> {
                let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(current, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(5));
                active.fetch_sub(1, Ordering::SeqCst);
                Ok(index)
            }) as SettledTask<usize, String>
        })
        .collect();
    let _ = umt_p_settled(tasks, Some(2));
    assert!(peak.load(Ordering::SeqCst) <= 2);
}

#[test]
fn test_p_settled_preserves_input_order() {
    let tasks: Vec<SettledTask<i32, String>> = vec![
        Box::new(|| {
            thread::sleep(Duration::from_millis(20));
            Ok(1)
        }),
        Box::new(|| Ok(2)),
    ];
    let results = umt_p_settled(tasks, None);
    let values: Vec<i32> = results
        .into_iter()
        .map(|r| match r {
            SettledResult::Fulfilled { value } => value,
            SettledResult::Rejected { .. } => -1,
        })
        .collect();
    assert_eq!(values, vec![1, 2]);
}

#[test]
fn test_p_settled_treats_limit_of_zero_as_unlimited() {
    let tasks: Vec<SettledTask<i32, String>> = vec![Box::new(|| Ok(1)), Box::new(|| Ok(2))];
    let results = umt_p_settled(tasks, Some(0));
    assert_eq!(results.len(), 2);
}
