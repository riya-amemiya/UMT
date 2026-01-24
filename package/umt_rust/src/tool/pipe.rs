use crate::error::{SafeResult, umt_safe_execute_mut};

/// A class to handle pipeline processing.
/// Allows chaining transformations in a fluent interface.
///
/// # Example
///
/// ```
/// use umt_rust::tool::Pipe;
///
/// let result = Pipe::new(5)
///     .map(|x| x + 1)
///     .map(|x| x * 2)
///     .end();
/// assert_eq!(result, 12);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Pipe<T> {
    value: T,
}

impl<T> Pipe<T> {
    /// Creates a new Pipe instance with an initial value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value for the pipe
    ///
    /// # Returns
    ///
    /// A new Pipe instance containing the value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let pipe = Pipe::new(42);
    /// assert_eq!(pipe.end(), 42);
    /// ```
    #[inline]
    pub fn new(value: T) -> Self {
        Pipe { value }
    }

    /// Applies a transformation function and returns a new Pipe instance.
    ///
    /// # Arguments
    ///
    /// * `f` - Transformation function to apply
    ///
    /// # Returns
    ///
    /// A new Pipe instance with the transformed value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let result = Pipe::new(5)
    ///     .map(|x| x * 2)
    ///     .end();
    /// assert_eq!(result, 10);
    /// ```
    #[inline]
    pub fn map<U, F>(self, f: F) -> Pipe<U>
    where
        F: FnOnce(T) -> U,
    {
        Pipe::new(f(self.value))
    }

    /// Applies a transformation function only if the condition is met.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Condition function
    /// * `f` - Transformation function to apply if condition is met
    ///
    /// # Returns
    ///
    /// A new Pipe instance with the conditionally transformed value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let result = Pipe::new(5)
    ///     .when(|x| *x > 3, |x| x * 2)
    ///     .end();
    /// assert_eq!(result, 10);
    ///
    /// let result = Pipe::new(2)
    ///     .when(|x| *x > 3, |x| x * 2)
    ///     .end();
    /// assert_eq!(result, 2);
    /// ```
    #[inline]
    pub fn when<F, P>(self, predicate: P, f: F) -> Pipe<T>
    where
        P: FnOnce(&T) -> bool,
        F: FnOnce(T) -> T,
    {
        if predicate(&self.value) {
            Pipe::new(f(self.value))
        } else {
            self
        }
    }

    /// Executes a side effect without changing the value.
    ///
    /// # Arguments
    ///
    /// * `f` - Function to execute as a side effect
    ///
    /// # Returns
    ///
    /// The same Pipe instance (allows chaining)
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let mut side_effect = 0;
    /// let result = Pipe::new(5)
    ///     .tap(|x| { side_effect = *x; })
    ///     .end();
    /// assert_eq!(result, 5);
    /// assert_eq!(side_effect, 5);
    /// ```
    #[inline]
    pub fn tap<F>(self, f: F) -> Pipe<T>
    where
        F: FnOnce(&T),
    {
        f(&self.value);
        self
    }

    /// Terminates the pipeline and returns the final value.
    ///
    /// # Returns
    ///
    /// The final result of the pipeline processing
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let value = Pipe::new(42).end();
    /// assert_eq!(value, 42);
    /// ```
    #[inline]
    pub fn end(self) -> T {
        self.value
    }

    /// Gets a reference to the current value without consuming the Pipe.
    ///
    /// # Returns
    ///
    /// A reference to the current value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let pipe = Pipe::new(42);
    /// assert_eq!(pipe.value(), &42);
    /// ```
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T: Clone> Pipe<T> {
    /// Strictly filters the value based on a predicate function.
    /// Panics if the predicate returns false.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Condition function that determines if value should pass
    ///
    /// # Returns
    ///
    /// A new Pipe instance with the same value if the predicate passes
    ///
    /// # Panics
    ///
    /// Panics with "Value did not match filter predicate" if predicate returns false
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let result = Pipe::new(42)
    ///     .filter_strict(|x| *x > 0)
    ///     .map(|x| x + 1)
    ///     .end();
    /// assert_eq!(result, 43);
    /// ```
    #[inline]
    pub fn filter_strict<P>(self, predicate: P) -> Pipe<T>
    where
        P: FnOnce(&T) -> bool,
    {
        if predicate(&self.value) {
            self
        } else {
            panic!("Value did not match filter predicate");
        }
    }

    /// Filters the value based on a predicate function.
    /// Returns a default value if the predicate returns false.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Condition function that determines if value should pass
    /// * `default_value` - Default value to use if predicate returns false
    ///
    /// # Returns
    ///
    /// A new Pipe instance with either the original value or the default
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let result = Pipe::new(42)
    ///     .filter_with_default(|x| *x > 100, 0)
    ///     .end();
    /// assert_eq!(result, 0);
    ///
    /// let result = Pipe::new(42)
    ///     .filter_with_default(|x| *x > 0, 0)
    ///     .end();
    /// assert_eq!(result, 42);
    /// ```
    #[inline]
    pub fn filter_with_default<P>(self, predicate: P, default_value: T) -> Pipe<T>
    where
        P: FnOnce(&T) -> bool,
    {
        if predicate(&self.value) {
            self
        } else {
            Pipe::new(default_value)
        }
    }

    /// Filters the value based on a predicate function.
    /// Returns a SafeResult type containing either the value or an error.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Condition function that determines if value should pass
    ///
    /// # Returns
    ///
    /// A new Pipe instance containing a SafeResult
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipe;
    ///
    /// let result = Pipe::new(42)
    ///     .filter_result(|x| *x > 0)
    ///     .map(|result| {
    ///         if result.is_success() {
    ///             result.value().unwrap() + 1
    ///         } else {
    ///             0
    ///         }
    ///     })
    ///     .end();
    /// assert_eq!(result, 43);
    /// ```
    #[inline]
    pub fn filter_result<P>(self, predicate: P) -> Pipe<SafeResult<T, String>>
    where
        P: FnOnce(&T) -> bool,
    {
        let value = self.value;
        let passes = predicate(&value);

        if passes {
            let result = umt_safe_execute_mut(move || value);
            Pipe::new(result)
        } else {
            let result = umt_safe_execute_mut(|| -> T {
                panic!("Value did not match filter predicate")
            });
            Pipe::new(result)
        }
    }
}

/// Creates a new Pipe instance with an initial value.
///
/// This is a convenience function that creates a Pipe instance.
///
/// # Arguments
///
/// * `initial_value` - The initial value for the pipeline
///
/// # Returns
///
/// A new Pipe instance containing the initial value
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_pipe;
///
/// let result = umt_pipe(5)
///     .map(|x| x + 1)
///     .end();
/// assert_eq!(result, 6);
/// ```
#[inline]
pub fn umt_pipe<T>(initial_value: T) -> Pipe<T> {
    Pipe::new(initial_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: i32,
        name: String,
        age: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Post {
        id: i32,
        title: String,
        content: String,
        author: User,
    }

    #[test]
    fn test_creates_instance_with_constructor() {
        let pipe = Pipe::new(42);
        assert_eq!(pipe.end(), 42);
    }

    #[test]
    fn test_returns_initial_value_when_calling_end() {
        let result = umt_pipe(1).end();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_transforms_value_with_map() {
        let result = umt_pipe(1)
            .map(|x| x + 1)
            .end();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_chains_multiple_map_operations() {
        let result = umt_pipe(1)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .map(|x| x - 1)
            .end();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_applies_transformation_when_condition_is_true_in_when() {
        let result = umt_pipe(5)
            .when(|x| *x > 3, |x| x * 2)
            .end();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_skips_transformation_when_condition_is_false_in_when() {
        let result = umt_pipe(2)
            .when(|x| *x > 3, |x| x * 2)
            .end();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_executes_side_effect_and_preserves_original_value_with_tap() {
        let side_effect = Rc::new(RefCell::new(0));
        let side_effect_clone = side_effect.clone();

        let result = umt_pipe(5)
            .tap(move |x| {
                *side_effect_clone.borrow_mut() = *x;
            })
            .end();

        assert_eq!(result, 5);
        assert_eq!(*side_effect.borrow(), 5);
    }

    #[test]
    fn test_correctly_infers_types() {
        let number_pipe = umt_pipe(1);
        let string_pipe = number_pipe.map(|x| x.to_string());
        let result: String = string_pipe.end();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_handles_complex_type_inference_correctly() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            age: 30,
        };

        let post = umt_pipe(user)
            .map(|user| Post {
                id: 1,
                title: "First Post".to_string(),
                content: "Hello, world!".to_string(),
                author: user,
            })
            .end();

        assert_eq!(post, Post {
            id: 1,
            title: "First Post".to_string(),
            content: "Hello, world!".to_string(),
            author: User {
                id: 1,
                name: "John Doe".to_string(),
                age: 30,
            },
        });
    }

    #[test]
    fn test_correctly_infers_nested_generic_types() {
        let users = umt_pipe(Vec::<User>::new())
            .map(|mut users| {
                users.push(User { id: 1, name: "John Doe".to_string(), age: 30 });
                users
            })
            .map(|mut users| {
                users.push(User { id: 2, name: "Jane Smith".to_string(), age: 25 });
                users
            })
            .end();

        assert_eq!(users, vec![
            User { id: 1, name: "John Doe".to_string(), age: 30 },
            User { id: 2, name: "Jane Smith".to_string(), age: 25 },
        ]);
    }

    #[test]
    fn test_processes_option_none_correctly() {
        let result = umt_pipe(None::<i32>).end();
        assert_eq!(result, None);
    }

    #[test]
    fn test_processes_option_some_correctly() {
        let result = umt_pipe(Some(42)).end();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_processes_empty_strings_correctly() {
        let result = umt_pipe(String::new())
            .map(|x| format!("{}test", x))
            .map(|x| x.to_uppercase())
            .end();
        assert_eq!(result, "TEST");
    }

    #[test]
    fn test_processes_numeric_zero_correctly() {
        let result = umt_pipe(0)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .end();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_processes_boolean_transformations_correctly() {
        let result = umt_pipe(true)
            .map(|x| !x)
            .map(|x| x.to_string())
            .end();
        assert_eq!(result, "false");
    }

    #[test]
    fn test_processes_array_transformations_correctly() {
        let result = umt_pipe(vec![1, 2, 3, 4, 5])
            .map(|arr| arr.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>())
            .map(|arr| arr.into_iter().map(|x| x * 2).collect::<Vec<_>>())
            .end();
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_processes_complex_object_transformations_correctly() {
        #[derive(Debug, Clone, PartialEq)]
        struct Data {
            count: i32,
            items: Vec<String>,
        }

        let initial = Data { count: 0, items: vec![] };
        let result = umt_pipe(initial)
            .map(|data| Data { count: data.count + 1, ..data })
            .map(|data| Data {
                items: {
                    let mut items = data.items.clone();
                    items.push(format!("Item {}", data.count));
                    items
                },
                ..data
            })
            .end();

        assert_eq!(result, Data {
            count: 1,
            items: vec!["Item 1".to_string()],
        });
    }

    #[test]
    fn test_processes_multiple_type_transformations_in_chain() {
        let result = umt_pipe(123)
            .map(|x| x.to_string())
            .map(|x| x.chars().collect::<Vec<_>>())
            .map(|arr| arr.into_iter().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>())
            .end();

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_combines_when_and_map_for_complex_processing() {
        let result = umt_pipe(5)
            .map(|x| x + 2)
            .when(|x| *x > 5, |x| x * 2)
            .when(|x| *x < 5, |x| x - 1)
            .map(|x| x + 1)
            .end();

        assert_eq!(result, 15);
    }

    // filter_strict tests
    mod filter_strict_tests {
        use super::*;

        fn is_positive(x: &i32) -> bool {
            *x > 0
        }

        #[test]
        fn test_filters_and_narrows_type() {
            let result = umt_pipe(42)
                .filter_strict(is_positive)
                .map(|x| x + 1)
                .end();
            assert_eq!(result, 43);
        }

        #[test]
        #[should_panic(expected = "Value did not match filter predicate")]
        fn test_throws_error_when_filter_condition_is_not_met() {
            umt_pipe(-5)
                .filter_strict(is_positive)
                .end();
        }

        #[test]
        fn test_combines_filter_strict_with_when_and_map_operations() {
            let result = umt_pipe(5)
                .filter_strict(is_positive)
                .map(|x| x + 2)
                .when(|x| *x > 5, |x| x * 2)
                .end();

            assert_eq!(result, 14);
        }
    }

    // filter_with_default tests
    mod filter_with_default_tests {
        use super::*;

        fn is_positive(x: &i32) -> bool {
            *x > 0
        }

        #[test]
        fn test_returns_original_value_when_predicate_is_true() {
            let result = umt_pipe(42)
                .filter_with_default(is_positive, 0)
                .map(|x| x + 1)
                .end();
            assert_eq!(result, 43);
        }

        #[test]
        fn test_returns_default_value_when_predicate_is_false() {
            let result = umt_pipe(-5)
                .filter_with_default(is_positive, 0)
                .map(|x| x + 1)
                .end();
            assert_eq!(result, 1);
        }

        #[test]
        fn test_works_with_complex_types() {
            #[derive(Debug, Clone, PartialEq)]
            struct ValidUser {
                id: i32,
                name: String,
                verified: bool,
            }

            fn is_valid_user(user: &ValidUser) -> bool {
                user.verified
            }

            let invalid_user = ValidUser {
                id: 1,
                name: "John".to_string(),
                verified: false,
            };

            let default_user = ValidUser {
                id: 0,
                name: "Default".to_string(),
                verified: true,
            };

            let result = umt_pipe(invalid_user)
                .filter_with_default(is_valid_user, default_user)
                .map(|u| u.verified)
                .end();

            assert_eq!(result, true);
        }
    }

    // filter_result tests
    mod filter_result_tests {
        use super::*;

        fn is_positive(x: &i32) -> bool {
            *x > 0
        }

        #[test]
        fn test_returns_success_result_when_predicate_is_true() {
            let result = umt_pipe(42)
                .filter_result(is_positive)
                .map(|result| {
                    if result.is_success() {
                        result.value().unwrap() + 1
                    } else {
                        0
                    }
                })
                .end();
            assert_eq!(result, 43);
        }

        #[test]
        fn test_returns_error_result_when_predicate_is_false() {
            let result = umt_pipe(-5)
                .filter_result(is_positive)
                .map(|result| {
                    if result.is_error() {
                        result.error().unwrap().clone()
                    } else {
                        String::new()
                    }
                })
                .end();
            assert_eq!(result, "Value did not match filter predicate");
        }

        #[test]
        fn test_can_be_chained_with_other_operations() {
            let result = umt_pipe(5)
                .filter_result(is_positive)
                .map(|result| {
                    if result.is_success() {
                        result.value().unwrap() * 2
                    } else {
                        0
                    }
                })
                .map(|x| x + 1)
                .end();

            assert_eq!(result, 11);
        }

        #[test]
        fn test_handles_error_cases_gracefully() {
            fn process_value(value: i32) -> i32 {
                umt_pipe(value)
                    .filter_result(is_positive)
                    .map(|result| {
                        if result.is_success() {
                            result.value().unwrap() * 2
                        } else {
                            0
                        }
                    })
                    .end()
            }

            assert_eq!(process_value(10), 20);
            assert_eq!(process_value(-5), 0);
        }
    }
}
