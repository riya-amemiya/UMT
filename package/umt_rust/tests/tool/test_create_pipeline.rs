use umt_rust::tool::{Pipeline, umt_create_pipeline};

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
fn test_returns_initial_value_when_called_without_transformations() {
    let pipeline = umt_create_pipeline(1);
    assert_eq!(pipeline.get(), &1);
}

#[test]
fn test_returns_a_new_pipeline_instance_when_transforming() {
    let pipeline = umt_create_pipeline(1);
    let new_pipeline = pipeline.transform(|x| x + 1);
    // The new pipeline is a Pipeline struct
    assert_eq!(new_pipeline.get(), &2);
}

#[test]
fn test_returns_transformed_value_when_called_with_transformation() {
    let pipeline = umt_create_pipeline(1);
    let new_pipeline = pipeline.transform(|x| x + 1);
    assert_eq!(new_pipeline.into_value(), 2);
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
fn test_correctly_infers_function_types() {
    let pipeline = umt_create_pipeline(1);
    let new_pipeline = pipeline.transform(|x: i32| x.to_string());
    let result: String = new_pipeline.into_value();
    assert_eq!(result, "1");
}

#[test]
fn test_correctly_handles_complex_type_inference() {
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        age: 30,
    };

    let pipeline = umt_create_pipeline(user);

    let post: Post = pipeline
        .transform(|user: User| Post {
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
    let users: Vec<User> = umt_create_pipeline(Vec::<User>::new())
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
fn test_works_correctly_with_none_as_initial_value() {
    let pipeline = umt_create_pipeline(None::<i32>);
    assert_eq!(pipeline.get(), &None);
}

#[test]
fn test_works_correctly_with_some_as_initial_value() {
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

#[test]
fn test_pipeline_new_is_equivalent_to_umt_create_pipeline() {
    let p1 = Pipeline::new(42);
    let p2 = umt_create_pipeline(42);
    assert_eq!(p1.get(), p2.get());
}
