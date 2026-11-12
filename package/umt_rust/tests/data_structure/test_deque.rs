use umt_rust::data_structure::Deque;

fn collect(deque: &Deque<i32>) -> Vec<i32> {
    deque.iter().copied().collect()
}

#[test]
fn test_deque_starts_empty() {
    let mut deque: Deque<i32> = Deque::new();
    assert_eq!(deque.size(), 0);
    assert!(deque.is_empty());
    assert_eq!(deque.pop_front(), None);
    assert_eq!(deque.pop_back(), None);
    assert_eq!(deque.peek_front(), None);
    assert_eq!(deque.peek_back(), None);
    assert!(collect(&deque).is_empty());
}

#[test]
fn test_deque_default_is_empty() {
    let deque: Deque<i32> = Deque::default();
    assert!(deque.is_empty());
}

#[test]
fn test_deque_push_and_pop_back_like_a_stack() {
    let mut deque = Deque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    assert_eq!(deque.peek_back(), Some(&3));
    assert_eq!(deque.pop_back(), Some(3));
    assert_eq!(deque.pop_back(), Some(2));
    assert_eq!(deque.pop_back(), Some(1));
    assert!(deque.is_empty());
}

#[test]
fn test_deque_push_back_and_pop_front_like_a_queue() {
    let mut deque = Deque::new();
    deque.push_back("a");
    deque.push_back("b");
    assert_eq!(deque.peek_front(), Some(&"a"));
    assert_eq!(deque.pop_front(), Some("a"));
    assert_eq!(deque.pop_front(), Some("b"));
    assert_eq!(deque.pop_front(), None);
}

#[test]
fn test_deque_push_front() {
    let mut deque = Deque::new();
    deque.push_front(3);
    deque.push_front(2);
    deque.push_front(1);
    assert_eq!(collect(&deque), vec![1, 2, 3]);
    assert_eq!(deque.peek_front(), Some(&1));
    assert_eq!(deque.peek_back(), Some(&3));
    assert_eq!(deque.size(), 3);
    assert!(!deque.is_empty());
}

#[test]
fn test_deque_mixed_operations() {
    let mut deque = Deque::new();
    deque.push_back(2);
    deque.push_front(1);
    deque.push_back(3);
    deque.push_front(0);
    assert_eq!(collect(&deque), vec![0, 1, 2, 3]);
    assert_eq!(deque.pop_back(), Some(3));
    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(collect(&deque), vec![1, 2]);
}

#[test]
fn test_deque_many_elements_keep_order() {
    let mut deque = Deque::new();
    for index in 0..20 {
        deque.push_back(index);
    }
    for index in 1..=20 {
        deque.push_front(-index);
    }
    assert_eq!(deque.size(), 40);
    assert_eq!(collect(&deque), (-20..20).collect::<Vec<_>>());
}

#[test]
fn test_deque_clear() {
    let mut deque = Deque::new();
    for index in 0..30 {
        deque.push_back(index);
    }
    deque.clear();
    assert_eq!(deque.size(), 0);
    assert!(deque.is_empty());
    deque.push_front(1);
    assert_eq!(collect(&deque), vec![1]);
}

#[test]
fn test_deque_for_loop_iteration() {
    let mut deque = Deque::new();
    deque.push_back("b");
    deque.push_front("a");
    let mut values = Vec::new();
    for value in &deque {
        values.push(*value);
    }
    assert_eq!(values, vec!["a", "b"]);
}
