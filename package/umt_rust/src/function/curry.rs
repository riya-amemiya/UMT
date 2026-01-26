//! Curry functions for transforming multi-argument functions into chains of single-argument functions.

/// Curries a 0-argument function (identity wrapper).
#[inline]
pub fn umt_curry0<F, R>(f: F) -> impl Fn() -> R
where
    F: Fn() -> R,
{
    move || f()
}

/// Curries a 1-argument function (identity wrapper).
#[inline]
pub fn umt_curry1<F, A, R>(f: F) -> impl Fn(A) -> R
where
    F: Fn(A) -> R,
{
    move |a| f(a)
}

/// Curries a 2-argument function using Box for nested returns.
#[inline]
pub fn umt_curry2<F, A, B, R>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> R>
where
    F: Fn(A, B) -> R + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
    R: 'static,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| f(a.clone(), b))
    }
}

/// Curries a 3-argument function.
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_curry3<F, A, B, C, R>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> R>>
where
    F: Fn(A, B, C) -> R + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: 'static,
    R: 'static,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            Box::new(move |c: C| f(a.clone(), b.clone(), c))
        })
    }
}

/// Curries a 4-argument function.
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_curry4<F, A, B, C, D, R>(
    f: F,
) -> impl Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> Box<dyn Fn(D) -> R>>>
where
    F: Fn(A, B, C, D) -> R + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: 'static,
    R: 'static,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            Box::new(move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                Box::new(move |d: D| f(a.clone(), b.clone(), c.clone(), d))
            })
        })
    }
}

/// Curries a 5-argument function.
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_curry5<F, A, B, C, D, E, R>(
    f: F,
) -> impl Fn(A) -> Box<dyn Fn(B) -> Box<dyn Fn(C) -> Box<dyn Fn(D) -> Box<dyn Fn(E) -> R>>>>
where
    F: Fn(A, B, C, D, E) -> R + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: 'static,
    R: 'static,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            Box::new(move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                Box::new(move |d: D| {
                    let f = f.clone();
                    let a = a.clone();
                    let b = b.clone();
                    let c = c.clone();
                    let d = d.clone();
                    Box::new(move |e: E| f(a.clone(), b.clone(), c.clone(), d.clone(), e))
                })
            })
        })
    }
}

/// Curries a 6-argument function.
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_curry6<F, A, B, C, D, E, G, R>(
    f: F,
) -> impl Fn(
    A,
) -> Box<
    dyn Fn(B) -> Box<dyn Fn(C) -> Box<dyn Fn(D) -> Box<dyn Fn(E) -> Box<dyn Fn(G) -> R>>>>,
>
where
    F: Fn(A, B, C, D, E, G) -> R + Clone + 'static,
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
    D: Clone + 'static,
    E: Clone + 'static,
    G: 'static,
    R: 'static,
{
    move |a: A| {
        let f = f.clone();
        let a = a.clone();
        Box::new(move |b: B| {
            let f = f.clone();
            let a = a.clone();
            let b = b.clone();
            Box::new(move |c: C| {
                let f = f.clone();
                let a = a.clone();
                let b = b.clone();
                let c = c.clone();
                Box::new(move |d: D| {
                    let f = f.clone();
                    let a = a.clone();
                    let b = b.clone();
                    let c = c.clone();
                    let d = d.clone();
                    Box::new(move |e: E| {
                        let f = f.clone();
                        let a = a.clone();
                        let b = b.clone();
                        let c = c.clone();
                        let d = d.clone();
                        let e = e.clone();
                        Box::new(move |g: G| {
                            f(a.clone(), b.clone(), c.clone(), d.clone(), e.clone(), g)
                        })
                    })
                })
            })
        })
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
}
