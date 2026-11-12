use umt_rust::data_structure::{PriorityQueue, PriorityQueueItem};

#[test]
fn test_dequeue_moves_non_clone_heavy_values_in_priority_order() {
    let mut queue: PriorityQueue<String> = PriorityQueue::new();
    for (index, priority) in [5.0, 1.0, 9.0, 3.0, 7.0, 2.0, 8.0].iter().enumerate() {
        queue.enqueue(format!("item-{}", index), *priority);
    }
    let mut order = Vec::new();
    while let Some(value) = queue.dequeue() {
        order.push(value);
    }
    assert_eq!(
        order,
        vec![
            "item-2", "item-6", "item-4", "item-0", "item-3", "item-5", "item-1"
        ]
    );
    assert!(queue.is_empty());
}

#[test]
fn test_dequeue_empty_single_and_two_elements() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    assert_eq!(queue.dequeue(), None);
    queue.enqueue("only", 1.0);
    assert_eq!(queue.dequeue(), Some("only"));
    assert_eq!(queue.dequeue(), None);
    queue.enqueue("low", 1.0);
    queue.enqueue("high", 2.0);
    assert_eq!(queue.dequeue(), Some("high"));
    assert_eq!(queue.peek(), Some(&"low"));
    assert_eq!(queue.dequeue(), Some("low"));
}

#[test]
fn test_with_initial_min_priority_feeds_enqueue_back() {
    let mut queue = PriorityQueue::with_initial(vec![
        PriorityQueueItem::new("b", 4.0),
        PriorityQueueItem::new("c", -2.5),
        PriorityQueueItem::new("a", 7.0),
    ]);
    queue.enqueue_back("back");
    let priorities = queue.to_vec_with_priorities();
    let back = priorities.iter().find(|item| item.value == "back").unwrap();
    assert_eq!(back.priority, -3.5);
    assert_eq!(queue.dequeue(), Some("a"));
    assert_eq!(queue.dequeue(), Some("b"));
    assert_eq!(queue.dequeue(), Some("c"));
    assert_eq!(queue.dequeue(), Some("back"));
}

#[test]
fn test_with_initial_single_element_min_priority() {
    let mut queue = PriorityQueue::with_initial(vec![PriorityQueueItem::new("a", 3.0)]);
    queue.enqueue_back("back");
    assert_eq!(queue.dequeue(), Some("a"));
    assert_eq!(queue.peek_priority(), Some(2.0));
}

#[test]
fn test_with_initial_empty_min_priority_is_zero() {
    let mut queue: PriorityQueue<&str> = PriorityQueue::with_initial(vec![]);
    queue.enqueue_back("back");
    assert_eq!(queue.peek_priority(), Some(-1.0));
}

#[test]
fn test_with_initial_nan_first_priority_stays_nan() {
    let mut queue = PriorityQueue::with_initial(vec![
        PriorityQueueItem::new("nan", f64::NAN),
        PriorityQueueItem::new("one", 1.0),
    ]);
    queue.enqueue_back("back");
    let priorities = queue.to_vec_with_priorities();
    let back = priorities.iter().find(|item| item.value == "back").unwrap();
    assert!(back.priority.is_nan());
}

#[test]
fn test_with_initial_nan_later_priority_is_ignored() {
    let mut queue = PriorityQueue::with_initial(vec![
        PriorityQueueItem::new("one", 1.0),
        PriorityQueueItem::new("nan", f64::NAN),
    ]);
    queue.enqueue_back("back");
    let priorities = queue.to_vec_with_priorities();
    let back = priorities.iter().find(|item| item.value == "back").unwrap();
    assert_eq!(back.priority, 0.0);
}
