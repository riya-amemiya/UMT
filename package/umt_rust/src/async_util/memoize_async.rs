//! Memoizes a fallible operation with per-key deduplication of concurrent calls.
//!
//! The TypeScript `memoizeAsync` shares one in-flight promise per key and caches
//! resolved values. The std equivalent here runs the operation on the calling
//! thread: while one thread computes a key, other threads calling with the same
//! key block until it finishes and receive the same `Ok` or `Err`. `Ok` values
//! are cached; `Err` values are not, so the next call invokes the operation again.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

type Outcome<R, E> = Option<Result<R, E>>;

struct Flight<R, E> {
    outcome: Mutex<Option<Outcome<R, E>>>,
    ready: Condvar,
}

struct State<K, R, E> {
    cache: HashMap<K, R>,
    insertion_order: VecDeque<K>,
    pending: HashMap<K, Arc<Flight<R, E>>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A memoized fallible operation that deduplicates concurrent calls per key.
///
/// The struct is `Sync` when its type parameters are `Send`, so it can be
/// shared between threads through `Arc`.
///
/// # Type Parameters
///
/// * `A` - The argument type
/// * `K` - The cache key type, must implement `Hash + Eq + Clone`
/// * `R` - The success type, must implement `Clone`
/// * `E` - The error type, must implement `Clone`
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_memoize_async;
///
/// let memoized = umt_memoize_async(|n: i32| Ok::<i32, String>(n * 2), None);
/// assert_eq!(memoized.call(5), Ok(10));
/// assert_eq!(memoized.call(5), Ok(10));
/// assert_eq!(memoized.cache_size(), 1);
/// ```
pub struct MemoizedAsync<A, K, R, E> {
    func: Box<dyn Fn(A) -> Result<R, E> + Send + Sync>,
    resolver: Box<dyn Fn(&A) -> K + Send + Sync>,
    state: Mutex<State<K, R, E>>,
    max_size: Option<usize>,
}

impl<A, K, R, E> MemoizedAsync<A, K, R, E>
where
    K: Hash + Eq + Clone,
    R: Clone,
    E: Clone,
{
    /// Calls the memoized operation.
    ///
    /// Returns the cached value when present. Otherwise, if another thread is
    /// already computing the same key, blocks until it finishes and returns
    /// its result; if not, invokes the operation, caches an `Ok` value, and
    /// returns the result. If the operation panics, the panic is resumed on the
    /// calling thread and every waiting thread panics as well.
    pub fn call(&self, arg: A) -> Result<R, E> {
        let key = (self.resolver)(&arg);
        let (flight, is_leader) = {
            let mut state = lock(&self.state);
            if let Some(value) = state.cache.get(&key) {
                return Ok(value.clone());
            }
            match state.pending.get(&key) {
                Some(flight) => (Arc::clone(flight), false),
                None => {
                    let flight = Arc::new(Flight {
                        outcome: Mutex::new(None),
                        ready: Condvar::new(),
                    });
                    state.pending.insert(key.clone(), Arc::clone(&flight));
                    (flight, true)
                }
            }
        };

        if !is_leader {
            let mut outcome = lock(&flight.outcome);
            while outcome.is_none() {
                outcome = flight
                    .ready
                    .wait(outcome)
                    .unwrap_or_else(PoisonError::into_inner);
            }
            return match outcome.as_ref() {
                Some(Some(result)) => result.clone(),
                _ => panic!("memoized operation panicked"),
            };
        }

        let computed = catch_unwind(AssertUnwindSafe(|| (self.func)(arg)));
        {
            let mut state = lock(&self.state);
            state.pending.remove(&key);
            if let Ok(Ok(value)) = &computed {
                let is_full = self.max_size.is_some_and(|max| state.cache.len() >= max);
                let evicted = if is_full {
                    state.insertion_order.pop_front()
                } else {
                    None
                };
                if let Some(oldest) = evicted {
                    state.cache.remove(&oldest);
                }
                state.cache.insert(key.clone(), value.clone());
                state.insertion_order.push_back(key);
            }
        }

        match computed {
            Ok(result) => {
                *lock(&flight.outcome) = Some(Some(result.clone()));
                flight.ready.notify_all();
                result
            }
            Err(payload) => {
                *lock(&flight.outcome) = Some(None);
                flight.ready.notify_all();
                resume_unwind(payload)
            }
        }
    }

    /// Returns the current number of entries in the cache.
    pub fn cache_size(&self) -> usize {
        lock(&self.state).cache.len()
    }

    /// Returns whether the cache contains the given key.
    pub fn cache_has(&self, key: &K) -> bool {
        lock(&self.state).cache.contains_key(key)
    }

    /// Returns a clone of the cached value for the given key, if present.
    pub fn cache_get(&self, key: &K) -> Option<R> {
        lock(&self.state).cache.get(key).cloned()
    }

    /// Clears the entire cache. In-flight calls are not affected.
    pub fn cache_clear(&self) {
        let mut state = lock(&self.state);
        state.cache.clear();
        state.insertion_order.clear();
    }
}

/// Creates a memoized version of a fallible single-argument operation.
///
/// Concurrent calls with the same argument share one invocation, `Ok` values
/// are cached keyed by the argument, and `Err` values are not cached. When
/// `max_size` is specified, the oldest entry is evicted when the cache is full.
///
/// # Arguments
///
/// * `f` - The operation to memoize
/// * `max_size` - Optional maximum number of cache entries
///
/// # Returns
///
/// A `MemoizedAsync` that can be called with `.call(arg)`.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::thread;
/// use umt_rust::async_util::umt_memoize_async;
///
/// let calls = Arc::new(AtomicUsize::new(0));
/// let counter = Arc::clone(&calls);
/// let memoized = Arc::new(umt_memoize_async(
///     move |n: u64| {
///         counter.fetch_add(1, Ordering::SeqCst);
///         Ok::<u64, String>(n * n)
///     },
///     None,
/// ));
/// let handles: Vec<_> = (0..4)
///     .map(|_| {
///         let memoized = Arc::clone(&memoized);
///         thread::spawn(move || memoized.call(3))
///     })
///     .collect();
/// for handle in handles {
///     assert_eq!(handle.join().unwrap(), Ok(9));
/// }
/// assert_eq!(memoized.call(3), Ok(9));
/// assert!(calls.load(Ordering::SeqCst) >= 1);
/// ```
///
/// Errors are not cached:
///
/// ```
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use umt_rust::async_util::umt_memoize_async;
///
/// static CALLS: AtomicUsize = AtomicUsize::new(0);
/// let memoized = umt_memoize_async(
///     |n: i32| {
///         if CALLS.fetch_add(1, Ordering::SeqCst) == 0 {
///             Err("fail".to_string())
///         } else {
///             Ok(n)
///         }
///     },
///     None,
/// );
/// assert_eq!(memoized.call(1), Err("fail".to_string()));
/// assert_eq!(memoized.call(1), Ok(1));
/// ```
pub fn umt_memoize_async<F, A, R, E>(f: F, max_size: Option<usize>) -> MemoizedAsync<A, A, R, E>
where
    F: Fn(A) -> Result<R, E> + Send + Sync + 'static,
    A: Hash + Eq + Clone + Send + Sync + 'static,
    R: Clone,
    E: Clone,
{
    umt_memoize_async_with_resolver(f, |arg: &A| arg.clone(), max_size)
}

/// Creates a memoized version of a fallible operation with a custom key resolver.
///
/// Behaves like [`umt_memoize_async`], deriving the cache key from the
/// argument with `resolver`.
///
/// # Arguments
///
/// * `f` - The operation to memoize
/// * `resolver` - A function that derives a cache key from the argument
/// * `max_size` - Optional maximum number of cache entries
///
/// # Returns
///
/// A `MemoizedAsync` that can be called with `.call(arg)`.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_memoize_async_with_resolver;
///
/// let memoized = umt_memoize_async_with_resolver(
///     |(a, b): (i32, i32)| Ok::<i32, String>(a + b),
///     |&(a, b): &(i32, i32)| format!("{}-{}", a, b),
///     None,
/// );
/// assert_eq!(memoized.call((1, 2)), Ok(3));
/// assert!(memoized.cache_has(&"1-2".to_string()));
/// ```
pub fn umt_memoize_async_with_resolver<F, G, A, K, R, E>(
    f: F,
    resolver: G,
    max_size: Option<usize>,
) -> MemoizedAsync<A, K, R, E>
where
    F: Fn(A) -> Result<R, E> + Send + Sync + 'static,
    G: Fn(&A) -> K + Send + Sync + 'static,
    K: Hash + Eq + Clone,
    R: Clone,
    E: Clone,
{
    MemoizedAsync {
        func: Box::new(f),
        resolver: Box::new(resolver),
        state: Mutex::new(State {
            cache: HashMap::new(),
            insertion_order: VecDeque::new(),
            pending: HashMap::new(),
        }),
        max_size,
    }
}
