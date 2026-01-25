use std::cell::RefCell;
use std::rc::Rc;
use umt_rust::tool::{umt_pipe, Pipe};

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

// Direct constructor test
#[test]
fn test_creates_instance_with_constructor() {
    let pipe = Pipe::new(42);
    assert_eq!(pipe.end(), 42);
}

// Basic functionality tests
#[test]
fn test_returns_initial_value_when_calling_end() {
    let result = umt_pipe(1).end();
    assert_eq!(result, 1);
}

#[test]
fn test_transforms_value_with_map() {
    let result = umt_pipe(1).map(|x| x + 1).end();
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
    let result = umt_pipe(5).when(|x| *x > 3, |x| x * 2).end();
    assert_eq!(result, 10);
}

#[test]
fn test_skips_transformation_when_condition_is_false_in_when() {
    let result = umt_pipe(2).when(|x| *x > 3, |x| x * 2).end();
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
    let users = umt_pipe(Vec::<User>::new())
        .map(|mut users| {
            users.push(User {
                id: 1,
                name: "John Doe".to_string(),
                age: 30,
            });
            users
        })
        .map(|mut users| {
            users.push(User {
                id: 2,
                name: "Jane Smith".to_string(),
                age: 25,
            });
            users
        })
        .end();

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
    let result = umt_pipe(0).map(|x| x + 1).map(|x| x * 2).end();
    assert_eq!(result, 2);
}

#[test]
fn test_processes_boolean_transformations_correctly() {
    let result = umt_pipe(true).map(|x| !x).map(|x| x.to_string()).end();
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

    let initial = Data {
        count: 0,
        items: vec![],
    };
    let result = umt_pipe(initial)
        .map(|data| Data {
            count: data.count + 1,
            ..data
        })
        .map(|data| Data {
            items: {
                let mut items = data.items.clone();
                items.push(format!("Item {}", data.count));
                items
            },
            ..data
        })
        .end();

    assert_eq!(
        result,
        Data {
            count: 1,
            items: vec!["Item 1".to_string()],
        }
    );
}

#[test]
fn test_processes_multiple_type_transformations_in_chain() {
    let result = umt_pipe(123)
        .map(|x| x.to_string())
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|arr| {
            arr.into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
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
        let result = umt_pipe(42).filter_strict(is_positive).map(|x| x + 1).end();
        assert_eq!(result, 43);
    }

    #[test]
    #[should_panic(expected = "Value did not match filter predicate")]
    fn test_throws_error_when_filter_condition_is_not_met() {
        umt_pipe(-5).filter_strict(is_positive).end();
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

        assert!(result);
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

#[test]
fn test_value_method_returns_reference() {
    let pipe = Pipe::new(42);
    assert_eq!(pipe.value(), &42);
}
