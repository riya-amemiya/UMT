/// Lazily maps values from an iterable using a generator
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `f` - The mapping function, which receives the value and the index
///
/// # Returns
///
/// An iterator yielding mapped values
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_map;
/// let doubled: Vec<i32> = lazy_map(vec![1, 2, 3], |n, _| n * 2).collect();
/// assert_eq!(doubled, vec![2, 4, 6]);
/// ```
pub fn lazy_map<I, F, U>(iterable: I, mut f: F) -> impl Iterator<Item = U>
where
    I: IntoIterator,
    F: FnMut(I::Item, usize) -> U,
{
    iterable.into_iter().enumerate().map(move |(i, v)| f(v, i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_maps_values_lazily() {
        let result: Vec<i32> = lazy_map(vec![1, 2, 3], |n, _| n * 2).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_provides_index() {
        let result: Vec<String> =
            lazy_map(vec!["a", "b", "c"], |v, i| format!("{}-{}", v, i)).collect();
        assert_eq!(result, vec!["a-0", "b-1", "c-2"]);
    }

    #[test]
    fn test_handles_empty_iterable() {
        let result: Vec<i32> = lazy_map(vec![], |n: i32, _| n).collect();
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_evaluates_lazily() {
        let call_count = Rc::new(RefCell::new(0));
        let call_count_clone = call_count.clone();

        let mut iter = lazy_map(vec![1, 2, 3, 4, 5], move |n, _| {
            *call_count_clone.borrow_mut() += 1;
            n * 2
        });

        assert_eq!(*call_count.borrow(), 0);

        iter.next();
        assert_eq!(*call_count.borrow(), 1);

        iter.next();
        assert_eq!(*call_count.borrow(), 2);
    }

    #[test]
    fn test_works_with_string_chars() {
        let result: Vec<String> =
            lazy_map("abc".chars(), |c, _| c.to_uppercase().to_string()).collect();
        assert_eq!(result, vec!["A", "B", "C"]);
    }
}
