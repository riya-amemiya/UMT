/// A priority queue item that holds a value and its priority.
///
/// # Type Parameters
///
/// * `T` - The type of the value stored in the item
#[derive(Debug, Clone, PartialEq)]
pub struct PriorityQueueItem<T> {
    /// The value stored in the priority queue
    pub value: T,
    /// The priority of the item (higher values have higher priority)
    pub priority: f64,
}

impl<T> PriorityQueueItem<T> {
    /// Creates a new PriorityQueueItem with the given value and priority.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store
    /// * `priority` - The priority value
    ///
    /// # Returns
    ///
    /// A new PriorityQueueItem instance
    pub fn new(value: T, priority: f64) -> Self {
        Self { value, priority }
    }
}

/// A priority queue implementation using a binary heap.
/// Higher priority values are dequeued first.
///
/// # Features
///
/// - **enqueue(value, priority)**: Add element with priority
/// - **enqueue_back(value)**: Add element to the back with lowest priority
/// - **dequeue()**: Remove and return highest priority element
/// - **peek()**: View highest priority element without removing
/// - **peek_priority()**: View highest priority value
/// - **size()**: Get number of elements
/// - **is_empty()**: Check if queue is empty
/// - **clear()**: Remove all elements
/// - **to_vec()**: Get all elements as vector
/// - **to_vec_with_priorities()**: Get all elements with priorities
///
/// # Time Complexity
///
/// - enqueue: O(log n)
/// - enqueue_back: O(log n)
/// - dequeue: O(log n)
/// - peek: O(1)
/// - peek_priority: O(1)
///
/// # Examples
///
/// ```
/// use umt_rust::data_structure::PriorityQueue;
///
/// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
/// queue.enqueue("low", 1.0);
/// queue.enqueue("high", 3.0);
/// queue.enqueue("medium", 2.0);
///
/// assert_eq!(queue.dequeue(), Some("high"));
/// assert_eq!(queue.dequeue(), Some("medium"));
/// assert_eq!(queue.dequeue(), Some("low"));
/// ```
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the queue
#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
    heap: Vec<PriorityQueueItem<T>>,
    min_priority: f64,
}

impl<T: Clone> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> PriorityQueue<T> {
    /// Creates a new empty PriorityQueue.
    ///
    /// # Returns
    ///
    /// A new empty PriorityQueue instance
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let queue: PriorityQueue<String> = PriorityQueue::new();
    /// assert!(queue.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            min_priority: 0.0,
        }
    }

    /// Creates a new PriorityQueue with initial elements.
    ///
    /// # Arguments
    ///
    /// * `initial_elements` - A vector of PriorityQueueItem elements
    ///
    /// # Returns
    ///
    /// A new PriorityQueue instance containing the initial elements
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::{PriorityQueue, PriorityQueueItem};
    ///
    /// let initial = vec![
    ///     PriorityQueueItem::new("low", 1.0),
    ///     PriorityQueueItem::new("high", 3.0),
    ///     PriorityQueueItem::new("medium", 2.0),
    /// ];
    /// let queue = PriorityQueue::with_initial(initial);
    /// assert_eq!(queue.size(), 3);
    /// assert_eq!(queue.peek(), Some(&"high"));
    /// ```
    pub fn with_initial(initial_elements: Vec<PriorityQueueItem<T>>) -> Self {
        let mut queue = Self {
            heap: initial_elements,
            min_priority: 0.0,
        };
        queue.update_min_priority();
        queue.build_heap();
        queue
    }

    /// Returns the number of elements in the queue.
    ///
    /// # Returns
    ///
    /// The number of elements in the queue
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("item", 1.0);
    /// assert_eq!(queue.size(), 1);
    /// ```
    #[inline]
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    ///
    /// True if the queue is empty, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// assert!(queue.is_empty());
    /// queue.enqueue("item", 1.0);
    /// assert!(!queue.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Adds an element to the queue with a specified priority.
    /// Higher priority values are dequeued first.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add
    /// * `priority` - The priority value (higher values have higher priority)
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// queue.enqueue("medium", 5.0);
    /// assert_eq!(queue.dequeue(), Some("high"));
    /// ```
    pub fn enqueue(&mut self, value: T, priority: f64) {
        self.heap.push(PriorityQueueItem::new(value, priority));
        self.update_min_priority_on_add(priority);
        self.heapify_up(self.heap.len() - 1);
    }

    /// Adds an element to the end of the queue with lowest priority.
    /// This element will be dequeued last (FIFO for equal lowest priority).
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("high", 10.0);
    /// queue.enqueue("medium", 5.0);
    /// queue.enqueue_back("back1");
    /// queue.enqueue_back("back2");
    /// assert_eq!(queue.dequeue(), Some("high"));
    /// assert_eq!(queue.dequeue(), Some("medium"));
    /// assert_eq!(queue.dequeue(), Some("back1"));
    /// assert_eq!(queue.dequeue(), Some("back2"));
    /// ```
    pub fn enqueue_back(&mut self, value: T) {
        let new_priority = self.min_priority - 1.0;
        self.heap.push(PriorityQueueItem::new(value, new_priority));
        self.min_priority = new_priority;
        self.heapify_up(self.heap.len() - 1);
    }

    /// Removes and returns the element with the highest priority.
    ///
    /// # Returns
    ///
    /// The element with highest priority, or None if queue is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// assert_eq!(queue.dequeue(), Some("high"));
    /// assert_eq!(queue.dequeue(), Some("low"));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        if self.heap.len() == 1 {
            return Some(self.heap.pop()?.value);
        }

        let result = self.heap[0].value.clone();
        self.heap[0] = self.heap.pop()?;
        self.heapify_down(0);
        Some(result)
    }

    /// Returns a reference to the element with the highest priority without removing it.
    ///
    /// # Returns
    ///
    /// A reference to the element with highest priority, or None if queue is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// assert_eq!(queue.peek(), Some(&"high"));
    /// assert_eq!(queue.size(), 2); // element not removed
    /// ```
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.heap.first().map(|item| &item.value)
    }

    /// Returns the priority of the element with the highest priority.
    ///
    /// # Returns
    ///
    /// The highest priority value, or None if queue is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// assert_eq!(queue.peek_priority(), Some(10.0));
    /// ```
    #[inline]
    pub fn peek_priority(&self) -> Option<f64> {
        self.heap.first().map(|item| item.priority)
    }

    /// Removes all elements from the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("item1", 1.0);
    /// queue.enqueue("item2", 2.0);
    /// assert_eq!(queue.size(), 2);
    /// queue.clear();
    /// assert_eq!(queue.size(), 0);
    /// assert!(queue.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
        self.min_priority = 0.0;
    }

    /// Returns a vector of all elements in the queue (without removing them).
    /// The order is not guaranteed to be sorted by priority.
    ///
    /// # Returns
    ///
    /// Vector of all elements in the queue
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// queue.enqueue("medium", 5.0);
    /// let array = queue.to_vec();
    /// assert_eq!(array.len(), 3);
    /// assert!(array.contains(&"low"));
    /// assert!(array.contains(&"high"));
    /// assert!(array.contains(&"medium"));
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        self.heap.iter().map(|item| item.value.clone()).collect()
    }

    /// Returns a vector of all elements with their priorities.
    /// The order is not guaranteed to be sorted by priority.
    ///
    /// # Returns
    ///
    /// Vector of all elements with their priorities
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::PriorityQueue;
    ///
    /// let mut queue: PriorityQueue<&str> = PriorityQueue::new();
    /// queue.enqueue("low", 1.0);
    /// queue.enqueue("high", 10.0);
    /// let array = queue.to_vec_with_priorities();
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn to_vec_with_priorities(&self) -> Vec<PriorityQueueItem<T>> {
        self.heap.clone()
    }

    /// Updates the minimum priority when adding elements.
    fn update_min_priority_on_add(&mut self, priority: f64) {
        if self.heap.len() == 1 || priority < self.min_priority {
            self.min_priority = priority;
        }
    }

    /// Updates the minimum priority from all elements (used in constructor).
    fn update_min_priority(&mut self) {
        if self.heap.is_empty() {
            self.min_priority = 0.0;
            return;
        }
        self.min_priority = self
            .heap
            .iter()
            .map(|item| item.priority)
            .fold(f64::INFINITY, f64::min);
    }

    /// Builds a max heap from the current heap array.
    fn build_heap(&mut self) {
        if self.heap.is_empty() {
            return;
        }
        for index in (0..=(self.heap.len() / 2).saturating_sub(1)).rev() {
            self.heapify_down(index);
        }
    }

    /// Moves an element up the heap to maintain heap property.
    fn heapify_up(&mut self, index: usize) {
        let mut current_index = index;
        while current_index > 0 {
            let parent_index = (current_index - 1) / 2;
            if self.heap[current_index].priority <= self.heap[parent_index].priority {
                break;
            }
            self.heap.swap(current_index, parent_index);
            current_index = parent_index;
        }
    }

    /// Moves an element down the heap to maintain heap property.
    fn heapify_down(&mut self, index: usize) {
        let mut current_index = index;
        loop {
            let left_child = 2 * current_index + 1;
            let right_child = 2 * current_index + 2;
            let mut largest = current_index;

            if left_child < self.heap.len()
                && self.heap[left_child].priority > self.heap[largest].priority
            {
                largest = left_child;
            }

            if right_child < self.heap.len()
                && self.heap[right_child].priority > self.heap[largest].priority
            {
                largest = right_child;
            }

            if largest == current_index {
                break;
            }

            self.heap.swap(current_index, largest);
            current_index = largest;
        }
    }
}
