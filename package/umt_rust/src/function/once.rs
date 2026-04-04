//! A function wrapper that ensures the underlying function is called only once.
//! Subsequent calls return the cached result of the first invocation.

use std::cell::RefCell;

/// Creates a function that is restricted to be called only once.
/// Subsequent calls return the result of the first invocation.
///
/// # Arguments
///
/// * `f` - The function to restrict
///
/// # Returns
///
/// A closure that invokes the original function only on its first call
/// and returns the cached result on subsequent calls.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_once;
///
/// let call_count = std::cell::Cell::new(0);
/// let once_fn = umt_once(|| {
///     call_count.set(call_count.get() + 1);
///     42
/// });
///
/// assert_eq!(once_fn(), 42);
/// assert_eq!(once_fn(), 42);
/// assert_eq!(call_count.get(), 1);
/// ```
pub fn umt_once<F, R>(f: F) -> impl Fn() -> R
where
    F: Fn() -> R,
    R: Clone,
{
    let result: RefCell<Option<R>> = RefCell::new(None);

    move || {
        let cached = result.borrow();
        if let Some(ref value) = *cached {
            return value.clone();
        }
        drop(cached);

        let value = f();
        *result.borrow_mut() = Some(value.clone());
        value
    }
}

/// Creates a two-argument function that is restricted to be called only once.
/// Subsequent calls return the result of the first invocation, ignoring new arguments.
///
/// # Arguments
///
/// * `f` - The function to restrict
///
/// # Returns
///
/// A closure that invokes the original function only on its first call
/// and returns the cached result on subsequent calls.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_once2;
///
/// let once_fn = umt_once2(|a: i32, b: i32| a + b);
/// assert_eq!(once_fn(3, 4), 7);
/// assert_eq!(once_fn(10, 20), 7);
/// ```
pub fn umt_once2<F, A, B, R>(f: F) -> impl Fn(A, B) -> R
where
    F: Fn(A, B) -> R,
    R: Clone,
{
    let result: RefCell<Option<R>> = RefCell::new(None);

    move |a: A, b: B| {
        let cached = result.borrow();
        if let Some(ref value) = *cached {
            return value.clone();
        }
        drop(cached);

        let value = f(a, b);
        *result.borrow_mut() = Some(value.clone());
        value
    }
}

/// Creates a one-argument function that is restricted to be called only once.
/// Subsequent calls return the result of the first invocation, ignoring new arguments.
///
/// # Arguments
///
/// * `f` - The function to restrict
///
/// # Returns
///
/// A closure that invokes the original function only on its first call
/// and returns the cached result on subsequent calls.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_once1;
///
/// let once_fn = umt_once1(|x: String| x.to_uppercase());
/// assert_eq!(once_fn("hello".to_string()), "HELLO");
/// assert_eq!(once_fn("world".to_string()), "HELLO");
/// ```
pub fn umt_once1<F, A, R>(f: F) -> impl Fn(A) -> R
where
    F: Fn(A) -> R,
    R: Clone,
{
    let result: RefCell<Option<R>> = RefCell::new(None);

    move |a: A| {
        let cached = result.borrow();
        if let Some(ref value) = *cached {
            return value.clone();
        }
        drop(cached);

        let value = f(a);
        *result.borrow_mut() = Some(value.clone());
        value
    }
}
