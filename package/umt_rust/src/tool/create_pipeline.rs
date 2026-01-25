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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_returns_initial_value_when_calling_get() {
        let pipeline = umt_create_pipeline(1);
        assert_eq!(pipeline.get(), &1);
    }

    #[test]
    fn test_returns_transformed_value() {
        let pipeline = umt_create_pipeline(1);
        let new_pipeline = pipeline.transform(|x| x + 1);
        assert_eq!(new_pipeline.get(), &2);
    }

    #[test]
    fn test_can_chain_multiple_transformations() {
        let result = umt_create_pipeline(1)
            .transform(|x| x + 1)
            .transform(|x| x * 2)
            .transform(|x| x - 1)
            .into_value();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_correctly_handles_type_transformations() {
        let result = umt_create_pipeline(1)
            .transform(|x: i32| x.to_string())
            .into_value();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_correctly_handles_complex_type_inference() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            age: 30,
        };

        let post = umt_create_pipeline(user)
            .transform(|user| Post {
                id: 1,
                title: "First Post".to_string(),
                content: "Hello, world!".to_string(),
                author: user,
            })
            .into_value();

        assert_eq!(
            post,
            Post {
                id: 1,
                title: "First Post".to_string(),
                content: "Hello, world!".to_string(),
                author: User {
                    id: 1,
                    name: "John Doe".to_string(),
                    age: 30,
                },
            }
        );
    }

    #[test]
    fn test_correctly_infers_nested_generic_types() {
        let users = umt_create_pipeline(Vec::<User>::new())
            .transform(|mut users| {
                users.push(User {
                    id: 1,
                    name: "John Doe".to_string(),
                    age: 30,
                });
                users
            })
            .transform(|mut users| {
                users.push(User {
                    id: 2,
                    name: "Jane Smith".to_string(),
                    age: 25,
                });
                users
            })
            .into_value();

        assert_eq!(
            users,
            vec![
                User {
                    id: 1,
                    name: "John Doe".to_string(),
                    age: 30
                },
                User {
                    id: 2,
                    name: "Jane Smith".to_string(),
                    age: 25
                },
            ]
        );
    }

    #[test]
    fn test_works_with_option_none() {
        let pipeline = umt_create_pipeline(None::<i32>);
        assert_eq!(pipeline.get(), &None);
    }

    #[test]
    fn test_works_with_option_some() {
        let pipeline = umt_create_pipeline(Some(42));
        assert_eq!(pipeline.get(), &Some(42));
    }

    #[test]
    fn test_can_process_empty_strings_correctly() {
        let result = umt_create_pipeline(String::new())
            .transform(|x| format!("{}test", x))
            .transform(|x| x.to_uppercase())
            .into_value();
        assert_eq!(result, "TEST");
    }

    #[test]
    fn test_can_process_numeric_zero_correctly() {
        let result = umt_create_pipeline(0)
            .transform(|x| x + 1)
            .transform(|x| x * 2)
            .into_value();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_can_process_boolean_transformations_correctly() {
        let result = umt_create_pipeline(true)
            .transform(|x| !x)
            .transform(|x| x.to_string())
            .into_value();
        assert_eq!(result, "false");
    }

    #[test]
    fn test_can_process_array_transformations_correctly() {
        let result = umt_create_pipeline(vec![1, 2, 3, 4, 5])
            .transform(|arr| arr.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>())
            .transform(|arr| arr.into_iter().map(|x| x * 2).collect::<Vec<_>>())
            .into_value();
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_can_process_complex_object_transformations_correctly() {
        #[derive(Debug, Clone, PartialEq)]
        struct Data {
            count: i32,
            items: Vec<String>,
        }

        let initial = Data {
            count: 0,
            items: vec![],
        };
        let result = umt_create_pipeline(initial)
            .transform(|data| Data {
                count: data.count + 1,
                ..data
            })
            .transform(|data| Data {
                items: {
                    let mut items = data.items.clone();
                    items.push(format!("Item {}", data.count));
                    items
                },
                ..data
            })
            .into_value();

        assert_eq!(
            result,
            Data {
                count: 1,
                items: vec!["Item 1".to_string()],
            }
        );
    }

    #[test]
    fn test_can_process_multiple_type_transformations_in_chain() {
        let result = umt_create_pipeline(123)
            .transform(|x: i32| x.to_string())
            .transform(|x: String| x.chars().collect::<Vec<_>>())
            .transform(|arr: Vec<char>| {
                arr.into_iter()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<_>>()
            })
            .into_value();

        assert_eq!(result, vec![1, 2, 3]);
    }
}
