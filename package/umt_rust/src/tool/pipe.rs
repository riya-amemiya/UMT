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
            let result =
                umt_safe_execute_mut(|| -> T { panic!("Value did not match filter predicate") });
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
