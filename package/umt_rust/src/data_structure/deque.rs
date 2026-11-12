use std::collections::VecDeque;
use std::collections::vec_deque::Iter;

/// A double-ended queue backed by a growable ring buffer.
/// Elements can be added and removed at both ends.
///
/// # Features
///
/// - **push_front(value)**: Add element to the front
/// - **push_back(value)**: Add element to the back
/// - **pop_front()**: Remove and return the front element
/// - **pop_back()**: Remove and return the back element
/// - **peek_front()**: View the front element without removing
/// - **peek_back()**: View the back element without removing
/// - **size()**: Get number of elements
/// - **is_empty()**: Check if deque is empty
/// - **clear()**: Remove all elements
/// - **iter()**: Iterate from front to back
///
/// # Time Complexity
///
/// - push_front / push_back: O(1) amortized
/// - pop_front / pop_back: O(1)
/// - peek_front / peek_back: O(1)
/// - size / is_empty: O(1)
///
/// # Examples
///
/// ```
/// use umt_rust::data_structure::Deque;
///
/// let mut deque: Deque<i32> = Deque::new();
/// deque.push_back(2);
/// deque.push_back(3);
/// deque.push_front(1);
///
/// assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_back(), Some(3));
/// assert_eq!(deque.size(), 1);
/// ```
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the deque
#[derive(Debug, Clone)]
pub struct Deque<T> {
    items: VecDeque<T>,
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deque<T> {
    /// Creates a new empty Deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let deque: Deque<String> = Deque::new();
    /// assert!(deque.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Returns the number of elements in the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("a");
    /// assert_eq!(deque.size(), 1);
    /// ```
    #[inline]
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Checks if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// assert!(deque.is_empty());
    /// deque.push_back("a");
    /// assert!(!deque.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Adds an element to the front of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_front(2);
    /// deque.push_front(1);
    /// assert_eq!(deque.peek_front(), Some(&1));
    /// ```
    #[inline]
    pub fn push_front(&mut self, value: T) {
        self.items.push_front(value);
    }

    /// Adds an element to the back of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.peek_back(), Some(&2));
    /// ```
    #[inline]
    pub fn push_back(&mut self, value: T) {
        self.items.push_back(value);
    }

    /// Removes and returns the front element, or None if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.pop_front(), Some(1));
    /// assert_eq!(deque.pop_front(), Some(2));
    /// assert_eq!(deque.pop_front(), None);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Removes and returns the back element, or None if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.pop_back(), Some(2));
    /// assert_eq!(deque.pop_back(), Some(1));
    /// assert_eq!(deque.pop_back(), None);
    /// ```
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    /// Returns a reference to the front element, or None if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.peek_front(), Some(&1));
    /// assert_eq!(deque.size(), 2);
    /// ```
    #[inline]
    pub fn peek_front(&self) -> Option<&T> {
        self.items.front()
    }

    /// Returns a reference to the back element, or None if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.peek_back(), Some(&2));
    /// assert_eq!(deque.size(), 2);
    /// ```
    #[inline]
    pub fn peek_back(&self) -> Option<&T> {
        self.items.back()
    }

    /// Removes all elements from the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.clear();
    /// assert_eq!(deque.size(), 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the elements from front to back.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::data_structure::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(2);
    /// deque.push_front(1);
    /// assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
