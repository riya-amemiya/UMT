//! A memoization wrapper that caches function results.
//! Results are stored in a HashMap keyed by the argument (or a custom resolver).
//! Supports optional max-size eviction using FIFO order.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

/// A memoized function that caches results keyed by the first argument.
///
/// # Type Parameters
///
/// * `A` - The argument type (used as cache key), must implement `Hash + Eq + Clone`
/// * `R` - The return type, must implement `Clone`
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_memoize;
///
/// let call_count = std::cell::Cell::new(0);
/// let memoized = umt_memoize(|n: i32| {
///     call_count.set(call_count.get() + 1);
///     n * 2
/// }, None);
///
/// assert_eq!(memoized(5), 10);
/// assert_eq!(memoized(5), 10);
/// assert_eq!(call_count.get(), 1);
/// assert_eq!(memoized.cache_size(), 1);
/// ```
pub struct Memoized<A, R> {
    func: Box<dyn Fn(A) -> R>,
    cache: Rc<RefCell<HashMap<A, R>>>,
    insertion_order: RefCell<VecDeque<A>>,
    max_size: Option<usize>,
}

impl<A, R> Memoized<A, R>
where
    A: Hash + Eq + Clone,
    R: Clone,
{
    /// Calls the memoized function. Returns the cached result if available,
    /// otherwise computes, caches, and returns the result.
    pub fn call(&self, arg: A) -> R {
        {
            let cache = self.cache.borrow();
            if let Some(value) = cache.get(&arg) {
                return value.clone();
            }
        }

        let result = (self.func)(arg.clone());

        let mut cache = self.cache.borrow_mut();
        if let Some(max) = self.max_size
            && cache.len() >= max
        {
            let mut order = self.insertion_order.borrow_mut();
            if let Some(oldest) = order.pop_front() {
                cache.remove(&oldest);
            }
        }
        cache.insert(arg.clone(), result.clone());
        self.insertion_order.borrow_mut().push_back(arg);

        result
    }

    /// Returns the current number of entries in the cache.
    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }

    /// Returns whether the cache contains the given key.
    pub fn cache_has(&self, key: &A) -> bool {
        self.cache.borrow().contains_key(key)
    }

    /// Returns a clone of the cached value for the given key, if present.
    pub fn cache_get(&self, key: &A) -> Option<R> {
        self.cache.borrow().get(key).cloned()
    }

    /// Clears the entire cache.
    pub fn cache_clear(&self) {
        self.cache.borrow_mut().clear();
        self.insertion_order.borrow_mut().clear();
    }
}

/// Creates a memoized version of the provided single-argument function.
///
/// Results are cached in a HashMap keyed by the argument. When `max_size`
/// is specified, the oldest entry is evicted when the cache exceeds the limit.
///
/// # Arguments
///
/// * `f` - The function to memoize
/// * `max_size` - Optional maximum number of cache entries
///
/// # Returns
///
/// A `Memoized` struct that can be called with `.call(arg)`.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_memoize;
///
/// let memoized = umt_memoize(|n: i32| n * 2, None);
/// assert_eq!(memoized.call(5), 10);
/// assert_eq!(memoized.call(5), 10);
/// assert_eq!(memoized.cache_size(), 1);
/// ```
///
/// With max size:
///
/// ```
/// use umt_rust::function::umt_memoize;
///
/// let memoized = umt_memoize(|n: i32| n * 2, Some(2));
/// memoized.call(1);
/// memoized.call(2);
/// memoized.call(3);
/// assert_eq!(memoized.cache_size(), 2);
/// assert!(!memoized.cache_has(&1));
/// assert!(memoized.cache_has(&2));
/// assert!(memoized.cache_has(&3));
/// ```
pub fn umt_memoize<F, A, R>(f: F, max_size: Option<usize>) -> Memoized<A, R>
where
    F: Fn(A) -> R + 'static,
    A: Hash + Eq + Clone,
    R: Clone,
{
    Memoized {
        func: Box::new(f),
        cache: Rc::new(RefCell::new(HashMap::new())),
        insertion_order: RefCell::new(VecDeque::new()),
        max_size,
    }
}

/// A memoized function with a custom key resolver for multi-argument functions.
///
/// # Type Parameters
///
/// * `A` - The argument type
/// * `K` - The cache key type, must implement `Hash + Eq + Clone`
/// * `R` - The return type, must implement `Clone`
pub struct MemoizedWithResolver<A, K, R> {
    func: Box<dyn Fn(A) -> R>,
    resolver: Box<dyn Fn(&A) -> K>,
    cache: Rc<RefCell<HashMap<K, R>>>,
    insertion_order: RefCell<VecDeque<K>>,
    max_size: Option<usize>,
}

impl<A, K, R> MemoizedWithResolver<A, K, R>
where
    K: Hash + Eq + Clone,
    R: Clone,
{
    /// Calls the memoized function using the custom resolver for cache keys.
    pub fn call(&self, arg: A) -> R {
        let key = (self.resolver)(&arg);
        {
            let cache = self.cache.borrow();
            if let Some(value) = cache.get(&key) {
                return value.clone();
            }
        }

        let result = (self.func)(arg);

        let mut cache = self.cache.borrow_mut();
        if let Some(max) = self.max_size
            && cache.len() >= max
        {
            let mut order = self.insertion_order.borrow_mut();
            if let Some(oldest) = order.pop_front() {
                cache.remove(&oldest);
            }
        }
        cache.insert(key.clone(), result.clone());
        self.insertion_order.borrow_mut().push_back(key);

        result
    }

    /// Returns the current number of entries in the cache.
    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }

    /// Returns whether the cache contains the given key.
    pub fn cache_has(&self, key: &K) -> bool {
        self.cache.borrow().contains_key(key)
    }

    /// Returns a clone of the cached value for the given key, if present.
    pub fn cache_get(&self, key: &K) -> Option<R> {
        self.cache.borrow().get(key).cloned()
    }

    /// Clears the entire cache.
    pub fn cache_clear(&self) {
        self.cache.borrow_mut().clear();
        self.insertion_order.borrow_mut().clear();
    }
}

/// Creates a memoized version of a function with a custom key resolver.
///
/// This is useful for multi-argument functions where a custom key
/// derivation strategy is needed.
///
/// # Arguments
///
/// * `f` - The function to memoize
/// * `resolver` - A function that derives a cache key from the argument
/// * `max_size` - Optional maximum number of cache entries
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_memoize_with_resolver;
///
/// let memoized = umt_memoize_with_resolver(
///     |(a, b): (i32, i32)| a + b,
///     |&(a, b): &(i32, i32)| format!("{}-{}", a, b),
///     None,
/// );
///
/// assert_eq!(memoized.call((1, 2)), 3);
/// assert_eq!(memoized.call((1, 2)), 3);
/// assert_eq!(memoized.cache_size(), 1);
/// ```
pub fn umt_memoize_with_resolver<F, A, K, R>(
    f: F,
    resolver: impl Fn(&A) -> K + 'static,
    max_size: Option<usize>,
) -> MemoizedWithResolver<A, K, R>
where
    F: Fn(A) -> R + 'static,
    K: Hash + Eq + Clone,
    R: Clone,
{
    MemoizedWithResolver {
        func: Box::new(f),
        resolver: Box::new(resolver),
        cache: Rc::new(RefCell::new(HashMap::new())),
        insertion_order: RefCell::new(VecDeque::new()),
        max_size,
    }
}
