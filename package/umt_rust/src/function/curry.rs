/// Curry functions for transforming multi-argument functions into chains of single-argument functions.
///
/// This module provides curry functions for different arities (number of arguments).
/// Due to Rust's static type system, we provide specific functions for each arity
/// rather than a single polymorphic curry function.
///
/// # Examples
///
/// ```
/// use umt_rust::function::{umt_curry2, umt_curry3};
///
/// // Curry a 2-argument function
/// let add = |a: i32, b: i32| a + b;
/// let curried_add = umt_curry2(add);
/// assert_eq!(curried_add(1)(2), 3);
///
/// // Curry a 3-argument function
/// let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
/// let curried = umt_curry3(multiply_add);
/// assert_eq!(curried(2)(3)(4), 10);
/// ```

/// Curries a 0-argument function (identity wrapper).
///
/// This is provided for completeness but simply wraps the function.
///
/// # Arguments
///
/// * `f` - A function that takes no arguments.
///
/// # Returns
///
/// A closure that, when called, returns the result of `f`.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry0;
///
/// let hello = || "hello";
/// let curried = umt_curry0(hello);
/// assert_eq!(curried(), "hello");
/// ```
#[inline]
pub fn umt_curry0<F, R>(f: F) -> impl Fn() -> R
where
    F: Fn() -> R,
{
    move || f()
}

/// Curries a 1-argument function (identity wrapper).
///
/// This is provided for completeness but simply wraps the function.
///
/// # Arguments
///
/// * `f` - A function that takes one argument.
///
/// # Returns
///
/// A closure that takes one argument and returns the result of `f`.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry1;
///
/// let double = |a: i32| a * 2;
/// let curried = umt_curry1(double);
/// assert_eq!(curried(5), 10);
/// ```
#[inline]
pub fn umt_curry1<F, A, R>(f: F) -> impl Fn(A) -> R
where
    F: Fn(A) -> R,
{
    move |a| f(a)
}

/// Curries a 2-argument function.
///
/// Transforms a function `(A, B) -> R` into `A -> B -> R`.
///
/// # Arguments
///
/// * `f` - A function that takes two arguments.
///
/// # Returns
///
/// A closure that takes the first argument and returns a closure
/// that takes the second argument and returns the result.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry2;
///
/// let add = |a: i32, b: i32| a + b;
/// let curried = umt_curry2(add);
/// assert_eq!(curried(2)(3), 5);
/// ```
#[inline]
pub fn umt_curry2<F, A, B, R>(f: F) -> impl Fn(A) -> impl Fn(B) -> R
where
    F: Fn(A, B) -> R + Clone,
    A: Clone,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        move |b: B| f(a.clone(), b)
    }
}

/// Curries a 3-argument function.
///
/// Transforms a function `(A, B, C) -> R` into `A -> B -> C -> R`.
///
/// # Arguments
///
/// * `f` - A function that takes three arguments.
///
/// # Returns
///
/// A chain of closures, each taking one argument.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry3;
///
/// let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
/// let curried = umt_curry3(multiply_add);
/// assert_eq!(curried(2)(3)(4), 10);
/// ```
#[inline]
pub fn umt_curry3<F, A, B, C, R>(f: F) -> impl Fn(A) -> impl Fn(B) -> impl Fn(C) -> R
where
    F: Fn(A, B, C) -> R + Clone,
    A: Clone,
    B: Clone,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            move |c: C| f(a.clone(), b.clone(), c)
        }
    }
}

/// Curries a 4-argument function.
///
/// Transforms a function `(A, B, C, D) -> R` into `A -> B -> C -> D -> R`.
///
/// # Arguments
///
/// * `f` - A function that takes four arguments.
///
/// # Returns
///
/// A chain of closures, each taking one argument.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry4;
///
/// let sum4 = |a: i32, b: i32, c: i32, d: i32| a + b + c + d;
/// let curried = umt_curry4(sum4);
/// assert_eq!(curried(1)(2)(3)(4), 10);
/// ```
#[inline]
pub fn umt_curry4<F, A, B, C, D, R>(
    f: F,
) -> impl Fn(A) -> impl Fn(B) -> impl Fn(C) -> impl Fn(D) -> R
where
    F: Fn(A, B, C, D) -> R + Clone,
    A: Clone,
    B: Clone,
    C: Clone,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                move |d: D| f(a.clone(), b.clone(), c.clone(), d)
            }
        }
    }
}

/// Curries a 5-argument function.
///
/// Transforms a function `(A, B, C, D, E) -> R` into `A -> B -> C -> D -> E -> R`.
///
/// # Arguments
///
/// * `f` - A function that takes five arguments.
///
/// # Returns
///
/// A chain of closures, each taking one argument.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry5;
///
/// let sum5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
/// let curried = umt_curry5(sum5);
/// assert_eq!(curried(1)(2)(3)(4)(5), 15);
/// ```
#[inline]
pub fn umt_curry5<F, A, B, C, D, E, R>(
    f: F,
) -> impl Fn(A) -> impl Fn(B) -> impl Fn(C) -> impl Fn(D) -> impl Fn(E) -> R
where
    F: Fn(A, B, C, D, E) -> R + Clone,
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                move |d: D| {
                    let f = f.clone();
                    let a = a.clone();
                    let b = b.clone();
                    let c = c.clone();
                    let d = d.clone();
                    move |e: E| f(a.clone(), b.clone(), c.clone(), d.clone(), e)
                }
            }
        }
    }
}

/// Curries a 6-argument function.
///
/// Transforms a function `(A, B, C, D, E, G) -> R` into `A -> B -> C -> D -> E -> G -> R`.
///
/// # Arguments
///
/// * `f` - A function that takes six arguments.
///
/// # Returns
///
/// A chain of closures, each taking one argument.
///
/// # Examples
///
/// ```
/// use umt_rust::function::umt_curry6;
///
/// let sum6 = |a: i32, b: i32, c: i32, d: i32, e: i32, g: i32| a + b + c + d + e + g;
/// let curried = umt_curry6(sum6);
/// assert_eq!(curried(1)(2)(3)(4)(5)(6), 21);
/// ```
#[inline]
pub fn umt_curry6<F, A, B, C, D, E, G, R>(
    f: F,
) -> impl Fn(A) -> impl Fn(B) -> impl Fn(C) -> impl Fn(D) -> impl Fn(E) -> impl Fn(G) -> R
where
    F: Fn(A, B, C, D, E, G) -> R + Clone,
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                move |d: D| {
                    let f = f.clone();
                    let a = a.clone();
                    let b = b.clone();
                    let c = c.clone();
                    let d = d.clone();
                    move |e: E| {
                        let f = f.clone();
                        let a = a.clone();
                        let b = b.clone();
                        let c = c.clone();
                        let d = d.clone();
                        let e = e.clone();
                        move |g: G| f(a.clone(), b.clone(), c.clone(), d.clone(), e.clone(), g)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curry0() {
        let hello = || "hello";
        let curried = umt_curry0(hello);
        assert_eq!(curried(), "hello");
    }

    #[test]
    fn test_curry1() {
        let double = |a: i32| a * 2;
        let curried = umt_curry1(double);
        assert_eq!(curried(5), 10);
    }

    #[test]
    fn test_curry2() {
        let add = |a: i32, b: i32| a + b;
        let curried = umt_curry2(add);
        assert_eq!(curried(2)(3), 5);
    }

    #[test]
    fn test_curry3() {
        let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
        let curried = umt_curry3(multiply_add);
        assert_eq!(curried(2)(3)(4), 10);
    }

    #[test]
    fn test_curry4() {
        let sum4 = |a: i32, b: i32, c: i32, d: i32| a + b + c + d;
        let curried = umt_curry4(sum4);
        assert_eq!(curried(1)(2)(3)(4), 10);
    }

    #[test]
    fn test_curry5() {
        let sum5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
        let curried = umt_curry5(sum5);
        assert_eq!(curried(1)(2)(3)(4)(5), 15);
    }

    #[test]
    fn test_curry6() {
        let sum6 = |a: i32, b: i32, c: i32, d: i32, e: i32, g: i32| a + b + c + d + e + g;
        let curried = umt_curry6(sum6);
        assert_eq!(curried(1)(2)(3)(4)(5)(6), 21);
    }

    #[test]
    fn test_curry_with_different_types() {
        let format_values = |a: i32, b: &str, c: bool| format!("{}{}{}", a, b, c);
        let curried = umt_curry3(format_values);
        assert_eq!(curried(1)("hello")(true), "1hellotrue");
    }

    #[test]
    fn test_curry_reusable() {
        let add = |a: i32, b: i32| a + b;
        let curried = umt_curry2(add);
        let add_five = curried(5);
        assert_eq!(add_five(3), 8);
        assert_eq!(add_five(10), 15);
    }

    #[test]
    fn test_curry_with_strings() {
        let concat = |a: String, b: String| format!("{}{}", a, b);
        let curried = umt_curry2(concat);
        assert_eq!(curried("Hello, ".to_string())("World!".to_string()), "Hello, World!");
    }

    #[test]
    fn test_curry_with_floats() {
        let multiply = |a: f64, b: f64| a * b;
        let curried = umt_curry2(multiply);
        let double = curried(2.0);
        assert!((double(3.5) - 7.0).abs() < f64::EPSILON);
    }
}
