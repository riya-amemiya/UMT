//! Integration tests for Function composition with Tool utilities
//!
//! Tests the interaction between function composition utilities:
//! - Currying functions with pipe operations
//! - Complex data transformation pipelines
//! - Functional programming patterns

use serde::Deserialize;
use umt_rust::function::{umt_curry2, umt_curry3};
use umt_rust::tool::{umt_create_pipeline, umt_parse_json, umt_pipe};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_combine_curry_with_pipe_for_data_transformations() {
        let add = |a: i32, b: i32| a + b;
        let multiply = |a: i32, b: i32| a * b;
        let to_string = |n: i32| n.to_string();

        let curried_add = umt_curry2(add);
        let curried_multiply = umt_curry2(multiply);

        let result = umt_pipe(5)
            .map(|x| curried_add(3)(x))
            .map(|x| curried_multiply(2)(x))
            .map(to_string)
            .end();

        assert_eq!(result, "16");
    }

    #[test]
    fn should_create_complex_data_processing_pipelines() {
        #[derive(Deserialize, Debug, Clone)]
        struct User {
            name: String,
            age: i32,
        }

        #[derive(Deserialize, Debug)]
        struct UserData {
            users: Vec<User>,
        }

        let json_data = r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#;

        let get_users = |data: UserData| data.users;
        let filter_adults = |users: Vec<User>| users.into_iter().filter(|u| u.age >= 25).collect::<Vec<_>>();
        let map_names = |users: Vec<User>| users.into_iter().map(|u| u.name).collect::<Vec<_>>();
        let join_names = |names: Vec<String>| names.join(", ");

        let result = umt_pipe(json_data)
            .map(|s| umt_parse_json::<UserData>(s).unwrap())
            .map(get_users)
            .map(filter_adults)
            .map(map_names)
            .map(join_names)
            .end();

        assert_eq!(result, "Alice, Bob");
    }

    #[test]
    fn should_handle_curried_functions_in_pipeline_operations() {
        let calculate = |operation: &str, a: i32, b: i32| -> i32 {
            match operation {
                "add" => a + b,
                "multiply" => a * b,
                "divide" => a / b,
                _ => 0,
            }
        };

        let curried_calculate = umt_curry3(calculate);
        let add_five = curried_calculate("add")(5);
        let multiply_by_three = curried_calculate("multiply")(3);

        let result = umt_create_pipeline(10)
            .transform(|n| add_five(n))
            .transform(|n| multiply_by_three(n))
            .transform(|n| n as f64 / 2.0)
            .into_value();

        assert_eq!(result, 22.5);
    }

    #[test]
    fn should_compose_mathematical_operations_with_data_parsing() {
        #[derive(Deserialize)]
        struct ValueData {
            value: i32,
        }

        let math_expressions = [
            r#"{"value": 10}"#,
            r#"{"value": 20}"#,
            r#"{"value": 30}"#,
        ];

        let parse_value = |json: &str| umt_parse_json::<ValueData>(json).unwrap().value;
        let square = |n: i32| n * n;
        let halve = |n: i32| n / 2;

        let process_expression = |expr: &str| -> i32 {
            umt_pipe(expr)
                .map(parse_value)
                .map(square)
                .map(halve)
                .end()
        };

        let results: Vec<_> = math_expressions.iter().map(|e| process_expression(e)).collect();
        assert_eq!(results, vec![50, 200, 450]);
    }

    #[test]
    fn should_handle_complex_nested_function_compositions() {
        #[derive(Debug, Clone, PartialEq)]
        struct TestObject {
            id: i32,
            name: String,
        }

        #[derive(Debug, Clone, PartialEq, Deserialize, serde::Serialize)]
        struct ProcessedObject {
            id: i32,
            name: String,
            processed: bool,
            timestamp: i64,
        }

        let transform_object = |obj: TestObject| -> ProcessedObject {
            ProcessedObject {
                id: obj.id,
                name: obj.name,
                processed: true,
                timestamp: 1234567890,
            }
        };

        let serialize = |obj: ProcessedObject| -> String { serde_json::to_string(&obj).unwrap() };
        let deserialize = |json: String| -> ProcessedObject {
            umt_parse_json::<ProcessedObject>(&json).unwrap()
        };

        let round_trip_transform = umt_create_pipeline(TestObject {
            id: 1,
            name: "test".to_string(),
        })
        .transform(transform_object)
        .transform(serialize)
        .transform(deserialize)
        .into_value();

        assert_eq!(round_trip_transform.id, 1);
        assert_eq!(round_trip_transform.name, "test");
        assert!(round_trip_transform.processed);
        assert_eq!(round_trip_transform.timestamp, 1234567890);
    }

    #[test]
    fn should_combine_currying_with_error_safe_operations() {
        let safe_divide = |a: i32, b: i32| -> Option<i32> {
            if b == 0 {
                None
            } else {
                Some(a / b)
            }
        };
        let safe_parse_number = |s: &str| -> Option<i32> { s.parse().ok() };

        let curried_safe_divide = std::rc::Rc::new(umt_curry2(safe_divide));
        let divide_by = std::rc::Rc::new(move |divisor: i32| {
            let curried = curried_safe_divide.clone();
            move |dividend: i32| (*curried)(dividend)(divisor)
        });

        let test_cases = ["10", "20", "invalid", "0"];

        let results: Vec<Option<i32>> = test_cases
            .iter()
            .map(|input| {
                let divide_by = divide_by.clone();
                umt_pipe(*input)
                    .map(safe_parse_number)
                    .map(move |num| num.and_then(|n| (*divide_by)(2)(n)))
                    .end()
            })
            .collect();

        assert_eq!(results, vec![Some(5), Some(10), None, Some(0)]);
    }

    #[test]
    fn should_create_reusable_transformation_pipelines() {
        let normalize_string = |s: String| s.trim().to_lowercase();
        let remove_spaces = |s: String| s.replace(' ', "");
        let add_prefix = umt_curry2(|prefix: &str, s: String| format!("{}_{}", prefix, s));

        let process_string = |input: &str| -> String {
            umt_create_pipeline(input.to_string())
                .transform(normalize_string)
                .transform(remove_spaces)
                .transform(|s| add_prefix("id")(s))
                .into_value()
        };

        let test_inputs = ["  Hello World  ", "Test String", "Another Example"];
        let identifiers: Vec<_> = test_inputs.iter().map(|s| process_string(s)).collect();

        assert_eq!(
            identifiers,
            vec!["id_helloworld", "id_teststring", "id_anotherexample"]
        );
    }

    #[test]
    fn should_handle_array_transformations_with_functional_composition() {
        let numbers = vec![1, 2, 3, 4, 5];

        let double = |n: i32| n * 2;
        let is_even = |n: &i32| n % 2 == 0;
        let sum = |arr: Vec<i32>| arr.into_iter().sum::<i32>();

        let process_array = umt_pipe(numbers)
            .map(|arr| arr.into_iter().map(double).collect::<Vec<_>>())
            .map(|arr| arr.into_iter().filter(is_even).collect::<Vec<_>>())
            .map(sum)
            .end();

        assert_eq!(process_array, 30);
    }
}
