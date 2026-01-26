/// A Pipeline that stores a value and can apply transformations to it.
///
/// This is a functional pipeline that allows chaining transformations.
/// Unlike the TypeScript version which uses function overloading,
/// the Rust version uses methods for clarity and type safety.
///
/// # Example
///
/// ```
/// use umt_rust::tool::Pipeline;
///
/// let pipeline = Pipeline::new(1);
/// assert_eq!(pipeline.get(), &1);
///
/// let result = Pipeline::new(1)
///     .transform(|x| x + 1)
///     .transform(|x| x * 2)
///     .transform(|x| x - 1)
///     .get()
///     .clone();
/// assert_eq!(result, 3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    /// Creates a new Pipeline with an initial value.
    ///
    /// # Arguments
    ///
    /// * `initial_value` - The initial value for the pipeline
    ///
    /// # Returns
    ///
    /// A new Pipeline instance containing the initial value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipeline;
    ///
    /// let pipeline = Pipeline::new(42);
    /// assert_eq!(pipeline.get(), &42);
    /// ```
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Pipeline {
            value: initial_value,
        }
    }

    /// Returns a reference to the stored value.
    ///
    /// # Returns
    ///
    /// A reference to the stored value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipeline;
    ///
    /// let pipeline = Pipeline::new("hello");
    /// assert_eq!(pipeline.get(), &"hello");
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Consumes the pipeline and returns the stored value.
    ///
    /// # Returns
    ///
    /// The stored value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipeline;
    ///
    /// let pipeline = Pipeline::new(42);
    /// let value = pipeline.into_value();
    /// assert_eq!(value, 42);
    /// ```
    #[inline]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Applies a transformation function and returns a new Pipeline with the result.
    ///
    /// # Arguments
    ///
    /// * `transformer` - A function that takes the current value and returns a new value
    ///
    /// # Returns
    ///
    /// A new Pipeline containing the transformed value
    ///
    /// # Example
    ///
    /// ```
    /// use umt_rust::tool::Pipeline;
    ///
    /// let result = Pipeline::new(5)
    ///     .transform(|x| x * 2)
    ///     .into_value();
    /// assert_eq!(result, 10);
    /// ```
    #[inline]
    pub fn transform<U, F>(self, transformer: F) -> Pipeline<U>
    where
        F: FnOnce(T) -> U,
    {
        Pipeline::new(transformer(self.value))
    }
}

/// Creates a new Pipeline with an initial value.
///
/// This is a convenience function that creates a Pipeline instance.
///
/// # Arguments
///
/// * `initial_value` - The initial value for the pipeline
///
/// # Returns
///
/// A new Pipeline instance containing the initial value
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_create_pipeline;
///
/// let pipeline = umt_create_pipeline(1);
/// assert_eq!(pipeline.get(), &1);
///
/// let result = umt_create_pipeline(1)
///     .transform(|x| x + 1)
///     .into_value();
/// assert_eq!(result, 2);
/// ```
#[inline]
pub fn umt_create_pipeline<T>(initial_value: T) -> Pipeline<T> {
    Pipeline::new(initial_value)
}
