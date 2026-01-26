//! Priority Queue integration tests.
//!
//! Ported from TypeScript: priorityQueue.test.ts

use umt_rust::data_structure::{PriorityQueue, PriorityQueueItem};

// ============================================================================
// Constructor Tests
// ============================================================================

#[test]
fn test_constructor_creates_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_constructor_creates_queue_with_initial_elements() {
    let initial_elements = vec![
        PriorityQueueItem::new("low", 1.0),
        PriorityQueueItem::new("high", 3.0),
        PriorityQueueItem::new("medium", 2.0),
    ];
    let queue = PriorityQueue::with_initial(initial_elements);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek(), Some(&"high"));
}

#[test]
fn test_constructor_creates_queue_with_empty_initial_elements() {
    let queue: PriorityQueue<String> = PriorityQueue::with_initial(vec![]);
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

// ============================================================================
// Enqueue Tests
// ============================================================================

#[test]
fn test_enqueue_adds_elements_to_queue() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 2.0);
    queue.enqueue("third", 3.0);

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());
}

#[test]
fn test_enqueue_maintains_priority_order() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.0);
    queue.enqueue("high", 3.0);
    queue.enqueue("medium", 2.0);

    assert_eq!(queue.peek(), Some(&"high"));
}

// ============================================================================
// Enqueue Back Tests
// ============================================================================

#[test]
fn test_enqueue_back_adds_elements_to_back() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("high", 10.0);
    queue.enqueue("medium", 5.0);
    queue.enqueue_back("back1");
    queue.enqueue_back("back2");

    assert_eq!(queue.size(), 4);
    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.dequeue(), Some("medium"));
    assert_eq!(queue.dequeue(), Some("back1"));
    assert_eq!(queue.dequeue(), Some("back2"));
}

#[test]
fn test_enqueue_back_works_with_empty_queue() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue_back("first");
    queue.enqueue_back("second");

    assert_eq!(queue.dequeue(), Some("first"));
    assert_eq!(queue.dequeue(), Some("second"));
}

#[test]
fn test_enqueue_back_maintains_fifo_order() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue_back("first");
    queue.enqueue_back("second");
    queue.enqueue_back("third");

    assert_eq!(queue.dequeue(), Some("first"));
    assert_eq!(queue.dequeue(), Some("second"));
    assert_eq!(queue.dequeue(), Some("third"));
}

#[test]
fn test_enqueue_back_places_after_priority_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue_back("back1");
    queue.enqueue("priority", 1.0);
    queue.enqueue_back("back2");

    assert_eq!(queue.dequeue(), Some("priority"));
    assert_eq!(queue.dequeue(), Some("back1"));
    assert_eq!(queue.dequeue(), Some("back2"));
}

// ============================================================================
// Dequeue Tests
// ============================================================================

#[test]
fn test_dequeue_returns_none_for_empty_queue() {
    let mut queue: PriorityQueue<String> = PriorityQueue::new();
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_dequeue_removes_and_returns_highest_priority() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.0);
    queue.enqueue("high", 3.0);
    queue.enqueue("medium", 2.0);

    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.dequeue(), Some("medium"));
    assert_eq!(queue.dequeue(), Some("low"));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_dequeue_handles_single_element_queue() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("single", 1.0);

    assert_eq!(queue.dequeue(), Some("single"));
    assert!(queue.is_empty());
}

#[test]
fn test_dequeue_handles_same_priority() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 1.0);
    queue.enqueue("third", 1.0);

    let results: Vec<_> = (0..3).filter_map(|_| queue.dequeue()).collect();
    assert!(results.contains(&"first"));
    assert!(results.contains(&"second"));
    assert!(results.contains(&"third"));
}

// ============================================================================
// Peek Tests
// ============================================================================

#[test]
fn test_peek_returns_none_for_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert_eq!(queue.peek(), None);
}

#[test]
fn test_peek_returns_highest_priority_without_removing() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.0);
    queue.enqueue("high", 3.0);
    queue.enqueue("medium", 2.0);

    assert_eq!(queue.peek(), Some(&"high"));
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek(), Some(&"high"));
}

// ============================================================================
// Peek Priority Tests
// ============================================================================

#[test]
fn test_peek_priority_returns_none_for_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert_eq!(queue.peek_priority(), None);
}

#[test]
fn test_peek_priority_returns_highest_priority_value() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.0);
    queue.enqueue("high", 3.0);
    queue.enqueue("medium", 2.0);

    assert_eq!(queue.peek_priority(), Some(3.0));
}

// ============================================================================
// Clear Tests
// ============================================================================

#[test]
fn test_clear_removes_all_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 2.0);
    queue.enqueue("third", 3.0);

    queue.clear();

    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
    assert_eq!(queue.peek(), None);
}

// ============================================================================
// ToVec Tests
// ============================================================================

#[test]
fn test_to_vec_returns_empty_for_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert!(queue.to_vec().is_empty());
}

#[test]
fn test_to_vec_returns_all_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 2.0);
    queue.enqueue("third", 3.0);

    let array = queue.to_vec();
    assert_eq!(array.len(), 3);
    assert!(array.contains(&"first"));
    assert!(array.contains(&"second"));
    assert!(array.contains(&"third"));
}

// ============================================================================
// ToVec With Priorities Tests
// ============================================================================

#[test]
fn test_to_vec_with_priorities_returns_empty_for_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert!(queue.to_vec_with_priorities().is_empty());
}

#[test]
fn test_to_vec_with_priorities_returns_all_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 2.0);
    queue.enqueue("third", 3.0);

    let array = queue.to_vec_with_priorities();
    assert_eq!(array.len(), 3);
    assert!(array.contains(&PriorityQueueItem::new("first", 1.0)));
    assert!(array.contains(&PriorityQueueItem::new("second", 2.0)));
    assert!(array.contains(&PriorityQueueItem::new("third", 3.0)));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_handles_negative_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("negative", -5.0);
    queue.enqueue("zero", 0.0);
    queue.enqueue("positive", 5.0);

    assert_eq!(queue.dequeue(), Some("positive"));
    assert_eq!(queue.dequeue(), Some("zero"));
    assert_eq!(queue.dequeue(), Some("negative"));
}

#[test]
fn test_handles_decimal_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.1);
    queue.enqueue("high", 1.9);
    queue.enqueue("medium", 1.5);

    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.dequeue(), Some("medium"));
    assert_eq!(queue.dequeue(), Some("low"));
}

#[test]
fn test_handles_zero_priority() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("zero", 0.0);
    queue.enqueue("positive", 1.0);
    queue.enqueue("negative", -1.0);

    assert_eq!(queue.dequeue(), Some("positive"));
    assert_eq!(queue.dequeue(), Some("zero"));
    assert_eq!(queue.dequeue(), Some("negative"));
}

#[test]
fn test_handles_very_large_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("max", f64::MAX);
    queue.enqueue("min", f64::MIN);
    queue.enqueue("normal", 1.0);

    assert_eq!(queue.dequeue(), Some("max"));
    assert_eq!(queue.dequeue(), Some("normal"));
    assert_eq!(queue.dequeue(), Some("min"));
}

#[test]
fn test_handles_infinity_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("positive-infinity", f64::INFINITY);
    queue.enqueue("negative-infinity", f64::NEG_INFINITY);
    queue.enqueue("normal", 1.0);

    assert_eq!(queue.dequeue(), Some("positive-infinity"));
    assert_eq!(queue.dequeue(), Some("normal"));
    assert_eq!(queue.dequeue(), Some("negative-infinity"));
}

// ============================================================================
// Data Type Compatibility Tests
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: String,
    description: String,
}

#[test]
fn test_works_with_custom_struct() {
    let mut queue: PriorityQueue<Task> = PriorityQueue::new();

    queue.enqueue(
        Task {
            id: "1".to_string(),
            description: "Low priority task".to_string(),
        },
        1.0,
    );
    queue.enqueue(
        Task {
            id: "2".to_string(),
            description: "High priority task".to_string(),
        },
        3.0,
    );
    queue.enqueue(
        Task {
            id: "3".to_string(),
            description: "Medium priority task".to_string(),
        },
        2.0,
    );

    let high_priority_task = queue.dequeue();
    assert!(high_priority_task.is_some());
    let task = high_priority_task.unwrap();
    assert_eq!(task.id, "2");
    assert_eq!(task.description, "High priority task");
}

#[test]
fn test_works_with_numbers() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();
    queue.enqueue(100, 1.0);
    queue.enqueue(300, 3.0);
    queue.enqueue(200, 2.0);

    assert_eq!(queue.dequeue(), Some(300));
    assert_eq!(queue.dequeue(), Some(200));
    assert_eq!(queue.dequeue(), Some(100));
}

#[test]
fn test_works_with_vectors() {
    let mut queue: PriorityQueue<Vec<i32>> = PriorityQueue::new();
    queue.enqueue(vec![1, 2, 3], 1.0);
    queue.enqueue(vec![4, 5, 6], 3.0);
    queue.enqueue(vec![7, 8, 9], 2.0);

    assert_eq!(queue.dequeue(), Some(vec![4, 5, 6]));
    assert_eq!(queue.dequeue(), Some(vec![7, 8, 9]));
    assert_eq!(queue.dequeue(), Some(vec![1, 2, 3]));
}

#[test]
fn test_works_with_option_values() {
    let mut queue: PriorityQueue<Option<&str>> = PriorityQueue::new();
    queue.enqueue(None, 1.0);
    queue.enqueue(Some("value"), 2.0);

    assert_eq!(queue.dequeue(), Some(Some("value")));
    assert_eq!(queue.dequeue(), Some(None));
}

// ============================================================================
// Complex Scenarios
// ============================================================================

#[test]
fn test_handles_mixed_operations() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    queue.enqueue(10, 1.0);
    queue.enqueue(30, 3.0);
    queue.enqueue(20, 2.0);

    assert_eq!(queue.dequeue(), Some(30));

    queue.enqueue(40, 4.0);
    queue.enqueue(15, 1.5);

    assert_eq!(queue.dequeue(), Some(40));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.dequeue(), Some(15));
    assert_eq!(queue.dequeue(), Some(10));
    assert!(queue.is_empty());
}

#[test]
fn test_handles_large_number_of_elements() {
    let mut queue: PriorityQueue<usize> = PriorityQueue::new();

    for i in 0..1000 {
        let priority = (i as f64) * 0.001;
        queue.enqueue(i, priority);
    }

    assert_eq!(queue.size(), 1000);

    let mut previous_priority = f64::INFINITY;
    while !queue.is_empty() {
        if let Some(current_priority) = queue.peek_priority() {
            assert!(
                current_priority <= previous_priority,
                "Priority order violation"
            );
            previous_priority = current_priority;
        }
        queue.dequeue();
    }
}

#[test]
fn test_task_scheduler_scenario() {
    #[derive(Debug, Clone)]
    struct ScheduledTask {
        name: String,
        priority: i32,
    }

    let mut queue: PriorityQueue<ScheduledTask> = PriorityQueue::new();

    queue.enqueue(
        ScheduledTask {
            name: "urgent".to_string(),
            priority: 10,
        },
        10.0,
    );
    queue.enqueue(
        ScheduledTask {
            name: "normal".to_string(),
            priority: 5,
        },
        5.0,
    );
    queue.enqueue(
        ScheduledTask {
            name: "low".to_string(),
            priority: 1,
        },
        1.0,
    );

    let first_task = queue.dequeue();
    assert!(first_task.is_some());
    let task = first_task.unwrap();
    assert_eq!(task.name, "urgent");
    assert_eq!(task.priority, 10);
}

#[test]
fn test_dynamic_priority_updates_via_reenqueue() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("task1", 1.0);
    queue.enqueue("task2", 2.0);
    queue.enqueue("task3", 3.0);

    let task = queue.dequeue();
    assert_eq!(task, Some("task3"));

    // Re-add with lower priority
    queue.enqueue("task3", 0.5);

    assert_eq!(queue.dequeue(), Some("task2"));
    assert_eq!(queue.dequeue(), Some("task1"));
    assert_eq!(queue.dequeue(), Some("task3"));
}

use umt_rust::data_structure::*;

#[test]
fn test_decimal_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("low", 1.1);
    queue.enqueue("high", 1.9);
    queue.enqueue("medium", 1.5);

    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.dequeue(), Some("medium"));
    assert_eq!(queue.dequeue(), Some("low"));
}

#[test]
fn test_default_trait() {
    let queue: PriorityQueue<i32> = PriorityQueue::default();
    assert!(queue.is_empty());
}

#[test]
fn test_dequeue_handles_single_element() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("single", 1.0);

    assert_eq!(queue.dequeue(), Some("single"));
    assert!(queue.is_empty());
}

#[test]
fn test_enqueue_adds_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("first", 1.0);
    queue.enqueue("second", 2.0);
    queue.enqueue("third", 3.0);

    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());
}

#[test]
fn test_enqueue_back_adds_to_back() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("high", 10.0);
    queue.enqueue("medium", 5.0);
    queue.enqueue_back("back1");
    queue.enqueue_back("back2");

    assert_eq!(queue.size(), 4);
    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.dequeue(), Some("medium"));
    assert_eq!(queue.dequeue(), Some("back1"));
    assert_eq!(queue.dequeue(), Some("back2"));
}

#[test]
fn test_infinity_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("positive-infinity", f64::INFINITY);
    queue.enqueue("negative-infinity", f64::NEG_INFINITY);
    queue.enqueue("normal", 1.0);

    assert_eq!(queue.dequeue(), Some("positive-infinity"));
    assert_eq!(queue.dequeue(), Some("normal"));
    assert_eq!(queue.dequeue(), Some("negative-infinity"));
}

#[test]
fn test_large_number_of_elements() {
    let mut queue: PriorityQueue<usize> = PriorityQueue::new();

    for i in 0..1000 {
        let priority = (i as f64) * 0.001;
        queue.enqueue(i, priority);
    }

    assert_eq!(queue.size(), 1000);

    let mut previous_priority = f64::INFINITY;
    while !queue.is_empty() {
        if let Some(current_priority) = queue.peek_priority() {
            assert!(current_priority <= previous_priority);
            previous_priority = current_priority;
        }
        queue.dequeue();
    }
}

#[test]
fn test_mixed_operations() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    queue.enqueue(10, 1.0);
    queue.enqueue(30, 3.0);
    queue.enqueue(20, 2.0);

    assert_eq!(queue.dequeue(), Some(30));

    queue.enqueue(40, 4.0);
    queue.enqueue(15, 1.5);

    assert_eq!(queue.dequeue(), Some(40));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.dequeue(), Some(15));
    assert_eq!(queue.dequeue(), Some(10));
    assert!(queue.is_empty());
}

#[test]
fn test_negative_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("negative", -5.0);
    queue.enqueue("zero", 0.0);
    queue.enqueue("positive", 5.0);

    assert_eq!(queue.dequeue(), Some("positive"));
    assert_eq!(queue.dequeue(), Some("zero"));
    assert_eq!(queue.dequeue(), Some("negative"));
}

#[test]
fn test_new_creates_empty_queue() {
    let queue: PriorityQueue<String> = PriorityQueue::new();
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_priority_queue_item_new() {
    let item = PriorityQueueItem::new("test", 5.0);
    assert_eq!(item.value, "test");
    assert_eq!(item.priority, 5.0);
}

#[test]
fn test_very_large_priorities() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("max", f64::MAX);
    queue.enqueue("min", f64::MIN);
    queue.enqueue("normal", 1.0);

    assert_eq!(queue.dequeue(), Some("max"));
    assert_eq!(queue.dequeue(), Some("normal"));
    assert_eq!(queue.dequeue(), Some("min"));
}

#[test]
fn test_with_initial_creates_queue_with_elements() {
    let initial = vec![
        PriorityQueueItem::new("low", 1.0),
        PriorityQueueItem::new("high", 3.0),
        PriorityQueueItem::new("medium", 2.0),
    ];
    let queue = PriorityQueue::with_initial(initial);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek(), Some(&"high"));
}

#[test]
fn test_with_initial_empty_elements() {
    let queue: PriorityQueue<String> = PriorityQueue::with_initial(vec![]);
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_with_numbers() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();
    queue.enqueue(100, 1.0);
    queue.enqueue(300, 3.0);
    queue.enqueue(200, 2.0);

    assert_eq!(queue.dequeue(), Some(300));
    assert_eq!(queue.dequeue(), Some(200));
    assert_eq!(queue.dequeue(), Some(100));
}

#[test]
fn test_with_option_values() {
    let mut queue: PriorityQueue<Option<&str>> = PriorityQueue::new();
    queue.enqueue(None, 1.0);
    queue.enqueue(Some("value"), 2.0);

    assert_eq!(queue.dequeue(), Some(Some("value")));
    assert_eq!(queue.dequeue(), Some(None));
}

#[test]
fn test_with_vectors() {
    let mut queue: PriorityQueue<Vec<i32>> = PriorityQueue::new();
    queue.enqueue(vec![1, 2, 3], 1.0);
    queue.enqueue(vec![4, 5, 6], 3.0);
    queue.enqueue(vec![7, 8, 9], 2.0);

    assert_eq!(queue.dequeue(), Some(vec![4, 5, 6]));
    assert_eq!(queue.dequeue(), Some(vec![7, 8, 9]));
    assert_eq!(queue.dequeue(), Some(vec![1, 2, 3]));
}

#[test]
fn test_zero_priority() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    queue.enqueue("zero", 0.0);
    queue.enqueue("positive", 1.0);
    queue.enqueue("negative", -1.0);

    assert_eq!(queue.dequeue(), Some("positive"));
    assert_eq!(queue.dequeue(), Some("zero"));
    assert_eq!(queue.dequeue(), Some("negative"));
}
