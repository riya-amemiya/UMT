//! A memoization wrapper that caches function results.
//! Results are stored in a HashMap keyed by the argument (or a custom resolver).
//! Supports optional max-size eviction using FIFO order and an optional
//! time to live per entry.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// A memoized function that caches results keyed by the first argument.
///
/// This is a thin wrapper over [`MemoizedWithResolver`] whose resolver clones
/// the argument.
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
/// let memoized = umt_memoize(|n: i32| n * 2, None);
///
/// assert_eq!(memoized.call(5), 10);
/// assert_eq!(memoized.call(5), 10);
/// assert_eq!(memoized.cache_size(), 1);
/// ```
pub struct Memoized<A, R> {
    inner: MemoizedWithResolver<A, A, R>,
}

impl<A, R> Memoized<A, R>
where
    A: Hash + Eq + Clone + 'static,
    R: Clone,
{
    /// Sets the time to live of each cache entry, like the TypeScript `ttl`
    /// option. An entry older than `ttl` is recomputed on its next access.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use umt_rust::function::umt_memoize;
    ///
    /// let memoized = umt_memoize(|n: i32| n * 2, None).with_ttl(Duration::from_millis(1000));
    /// assert_eq!(memoized.call(5), 10);
    /// assert_eq!(memoized.call(5), 10);
    /// ```
    pub fn with_ttl(self, ttl: Duration) -> Self {
        Memoized {
            inner: self.inner.with_ttl(ttl),
        }
    }

    /// Calls the memoized function. Returns the cached result if available
    /// and not expired, otherwise computes, caches, and returns the result.
    pub fn call(&self, arg: A) -> R {
        self.inner.call(arg)
    }

    /// Returns the current number of entries in the cache.
    pub fn cache_size(&self) -> usize {
        self.inner.cache_size()
    }

    /// Returns whether the cache contains the given key.
    pub fn cache_has(&self, key: &A) -> bool {
        self.inner.cache_has(key)
    }

    /// Returns a clone of the cached value for the given key, if present.
    pub fn cache_get(&self, key: &A) -> Option<R> {
        self.inner.cache_get(key)
    }

    /// Clears the entire cache.
    pub fn cache_clear(&self) {
        self.inner.cache_clear();
    }
}

/// Creates a memoized version of the provided single-argument function.
///
/// Results are cached in a HashMap keyed by the argument. When `max_size`
/// is specified, the oldest entry is evicted when the cache exceeds the limit.
/// Call [`Memoized::with_ttl`] to expire entries after a duration.
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
    A: Hash + Eq + Clone + 'static,
    R: Clone,
{
    Memoized {
        inner: umt_memoize_with_resolver(f, |arg: &A| arg.clone(), max_size),
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
    expirations: RefCell<HashMap<K, Instant>>,
    max_size: Option<usize>,
    ttl: Option<Duration>,
}

impl<A, K, R> MemoizedWithResolver<A, K, R>
where
    K: Hash + Eq + Clone,
    R: Clone,
{
    /// Sets the time to live of each cache entry, like the TypeScript `ttl`
    /// option. An entry older than `ttl` is recomputed on its next access.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use umt_rust::function::umt_memoize_with_resolver;
    ///
    /// let memoized = umt_memoize_with_resolver(
    ///     |(a, b): (i32, i32)| a + b,
    ///     |&(a, b): &(i32, i32)| format!("{}-{}", a, b),
    ///     None,
    /// )
    /// .with_ttl(Duration::from_millis(1000));
    /// assert_eq!(memoized.call((1, 2)), 3);
    /// ```
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    fn is_expired(&self, key: &K) -> bool {
        match self.ttl {
            Some(_) => self
                .expirations
                .borrow()
                .get(key)
                .is_none_or(|expiration| *expiration <= Instant::now()),
            None => false,
        }
    }

    fn remove_entry(&self, key: &K) {
        if self.cache.borrow_mut().remove(key).is_some() {
            self.insertion_order
                .borrow_mut()
                .retain(|existing| existing != key);
        }
        self.expirations.borrow_mut().remove(key);
    }

    /// Calls the memoized function using the custom resolver for cache keys.
    ///
    /// Returns the cached result if available and not expired, otherwise
    /// computes, caches, and returns the result.
    pub fn call(&self, arg: A) -> R {
        let key = (self.resolver)(&arg);
        let cached = self.cache.borrow().get(&key).cloned();
        if let Some(value) = cached {
            if !self.is_expired(&key) {
                return value;
            }
            self.remove_entry(&key);
        }

        let result = (self.func)(arg);

        let is_full = self
            .max_size
            .is_some_and(|max| self.cache.borrow().len() >= max);
        if is_full {
            let oldest = self.insertion_order.borrow_mut().pop_front();
            if let Some(oldest) = oldest {
                self.cache.borrow_mut().remove(&oldest);
                self.expirations.borrow_mut().remove(&oldest);
            }
        }
        if let Some(ttl) = self.ttl {
            self.expirations
                .borrow_mut()
                .insert(key.clone(), Instant::now() + ttl);
        }
        self.cache.borrow_mut().insert(key.clone(), result.clone());
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
        self.expirations.borrow_mut().clear();
    }
}

/// Creates a memoized version of a function with a custom key resolver.
///
/// This is useful for multi-argument functions where a custom key
/// derivation strategy is needed. Call [`MemoizedWithResolver::with_ttl`] to
/// expire entries after a duration.
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
        expirations: RefCell::new(HashMap::new()),
        max_size,
        ttl: None,
    }
}
