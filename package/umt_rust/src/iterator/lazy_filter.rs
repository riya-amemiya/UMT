/// Lazily filters values from an iterable using a generator
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `predicate` - The filter predicate, which receives a reference to the value and the index
///
/// # Returns
///
/// An iterator yielding filtered values
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_filter;
/// let evens: Vec<i32> = lazy_filter(vec![1, 2, 3, 4], |n, _| n % 2 == 0).collect();
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn lazy_filter<I, F>(iterable: I, mut predicate: F) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item, usize) -> bool,
{
    iterable.into_iter().enumerate().filter_map(
        move |(i, v)| {
            if predicate(&v, i) { Some(v) } else { None }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_filters_values_lazily() {
        let result: Vec<i32> = lazy_filter(vec![1, 2, 3, 4, 5], |n, _| n % 2 == 0).collect();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_provides_index() {
        let result: Vec<i32> = lazy_filter(vec![10, 20, 30, 40], |_, i| i >= 2).collect();
        assert_eq!(result, vec![30, 40]);
    }

    #[test]
    fn test_handles_empty_iterable() {
        let result: Vec<i32> = lazy_filter(vec![], |n: &i32, _| *n > 0).collect();
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_handles_no_matching_elements() {
        let result: Vec<i32> = lazy_filter(vec![1, 2, 3], |_, _| false).collect();
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_handles_all_matching_elements() {
        let result: Vec<i32> = lazy_filter(vec![1, 2, 3], |_, _| true).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_evaluates_lazily() {
        let call_count = Rc::new(RefCell::new(0));
        let call_count_clone = call_count.clone();

        let mut iter = lazy_filter(vec![1, 2, 3, 4, 5], move |n, _| {
            *call_count_clone.borrow_mut() += 1;
            *n > 3
        });

        assert_eq!(*call_count.borrow(), 0);

        iter.next();
        // 1 (fail), 2 (fail), 3 (fail), 4 (pass). Should be 4 calls.
        assert_eq!(*call_count.borrow(), 4);
    }
}
