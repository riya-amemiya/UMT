//! Integration tests for Error handling with other modules
//!
//! Tests the interaction between error handling and other utilities:
//! - Safe execution with mathematical operations
//! - Error handling in data parsing pipelines
//! - Validation with error recovery

use serde::Deserialize;
use umt_rust::error::{umt_safe_execute, umt_safe_execute_mut};
use umt_rust::tool::{umt_parse_json, umt_pipe};
use umt_rust::validate::string::umt_validate_email_validator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_calculation_errors_safely() {
        // Valid calculation
        let valid_result = umt_safe_execute(|| {
            let a = 10;
            let b = 5;
            (a + b).to_string()
        });
        assert!(valid_result.is_success());
        assert_eq!(valid_result.value(), Some(&"15".to_string()));

        // Invalid (simulated error)
        let invalid_result = umt_safe_execute(|| -> String { panic!("Invalid expression") });
        assert!(invalid_result.is_error());
    }

    #[test]
    fn should_handle_json_parsing_errors_in_data_processing_pipelines() {
        #[derive(Deserialize, Debug)]
        struct UserData {
            name: Option<String>,
            age: Option<serde_json::Value>,
        }

        let json_strings = [
            r#"{"name": "Alice", "age": 30}"#,
            "invalid json",
            r#"{"name": "Bob"}"#,
            r#"{"age": "not a number"}"#,
        ];

        #[derive(Debug, PartialEq)]
        struct ProcessedData {
            error: Option<String>,
            name: String,
            age: i32,
            valid: bool,
        }

        let process_json_safely = |json_str: &str| -> ProcessedData {
            let parse_result = umt_safe_execute(|| umt_parse_json::<UserData>(json_str));

            if parse_result.is_error() {
                return ProcessedData {
                    error: Some("Invalid JSON".to_string()),
                    name: "Unknown".to_string(),
                    age: 0,
                    valid: false,
                };
            }

            let data = parse_result.value().unwrap();
            match data {
                Ok(user_data) => {
                    let name = user_data.name.clone().unwrap_or_else(|| "Unknown".to_string());
                    let age = user_data
                        .age
                        .as_ref()
                        .and_then(|a| a.as_i64())
                        .map(|a| a as i32)
                        .unwrap_or(0);
                    let valid = user_data.name.is_some() && user_data.age.as_ref().map(|a| a.is_number()).unwrap_or(false);

                    ProcessedData {
                        error: None,
                        name,
                        age,
                        valid,
                    }
                }
                Err(_) => ProcessedData {
                    error: Some("Invalid JSON".to_string()),
                    name: "Unknown".to_string(),
                    age: 0,
                    valid: false,
                },
            }
        };

        let results: Vec<_> = json_strings.iter().map(|s| process_json_safely(s)).collect();

        assert!(results[0].valid);
        assert_eq!(results[1].error, Some("Invalid JSON".to_string()));
        assert!(!results[2].valid);
        assert!(!results[3].valid);
    }

    #[test]
    fn should_create_error_resilient_data_transformation_pipelines() {
        #[derive(Deserialize, Debug)]
        struct UserScore {
            email: String,
            score: String,
        }

        let user_data = [
            r#"{"email": "alice@example.com", "score": "85"}"#,
            r#"{"email": "invalid-email", "score": "90"}"#,
            "invalid json",
            r#"{"email": "bob@test.com", "score": "not-a-number"}"#,
        ];

        #[derive(Debug)]
        #[allow(dead_code)]
        struct ProcessResult {
            success: bool,
            error: Option<String>,
            email: String,
            score: f64,
        }

        let process_user_data = |json_str: &str| -> ProcessResult {
            let parse_result = umt_safe_execute(|| umt_parse_json::<UserScore>(json_str));

            if parse_result.is_error() {
                return ProcessResult {
                    success: false,
                    error: Some("Parse error".to_string()),
                    email: String::new(),
                    score: 0.0,
                };
            }

            let data = parse_result.value().unwrap();
            match data {
                Ok(user_score) => {
                    let email_validator = umt_validate_email_validator(None, None);
                    let email_valid = (email_validator.validate)(&user_score.email);

                    let score_result = umt_safe_execute(|| -> f64 {
                        let num: f64 = user_score.score.parse().unwrap();
                        if num.is_nan() {
                            panic!("Invalid score");
                        }
                        num
                    });

                    let score = if score_result.is_success() {
                        *score_result.value().unwrap()
                    } else {
                        0.0
                    };

                    let error = if !email_valid {
                        Some("Invalid email".to_string())
                    } else if score_result.is_error() {
                        Some("Invalid score".to_string())
                    } else {
                        None
                    };

                    ProcessResult {
                        success: email_valid && score_result.is_success(),
                        error,
                        email: user_score.email.clone(),
                        score,
                    }
                }
                Err(_) => ProcessResult {
                    success: false,
                    error: Some("Parse error".to_string()),
                    email: String::new(),
                    score: 0.0,
                },
            }
        };

        let results: Vec<_> = user_data.iter().map(|s| process_user_data(s)).collect();

        assert!(results[0].success);
        assert_eq!(results[0].score, 85.0);

        assert!(!results[1].success);
        assert_eq!(results[1].error, Some("Invalid email".to_string()));

        assert!(!results[2].success);
        assert_eq!(results[2].error, Some("Parse error".to_string()));

        assert!(!results[3].success);
        assert_eq!(results[3].error, Some("Invalid score".to_string()));
    }

    #[test]
    fn should_handle_mathematical_operations_with_fallback_values() {
        let calculate_with_fallback = |valid: bool, a: i32, b: i32, fallback: i32| -> (i32, bool) {
            let calc_result = umt_safe_execute(|| -> i32 {
                if !valid {
                    panic!("Invalid expression");
                }
                a + b
            });

            if calc_result.is_error() {
                return (fallback, true);
            }

            let value = *calc_result.value().unwrap();
            (value, false)
        };

        let expressions = [
            (true, 10, 5, 15),   // valid
            (false, 0, 0, 0),   // invalid
            (true, 20, 4, 24),  // valid division result would be 5
        ];

        let results: Vec<_> = expressions
            .iter()
            .map(|(valid, a, b, _)| calculate_with_fallback(*valid, *a, *b, 0))
            .collect();

        assert_eq!(results[0].0, 15);
        assert!(!results[0].1);

        assert_eq!(results[1].0, 0);
        assert!(results[1].1);

        assert_eq!(results[2].0, 24);
        assert!(!results[2].1);
    }

    #[test]
    fn should_chain_safe_operations_in_complex_workflows() {
        #[derive(Deserialize, Debug, Clone)]
        struct OperationData {
            operation: String,
            values: Vec<i64>,
        }

        let process_complex_data = |input: &str| -> Option<i64> {
            umt_pipe(input)
                .map(|json_str| {
                    let result = umt_safe_execute(|| umt_parse_json::<OperationData>(json_str));
                    if result.is_success() {
                        result.value().unwrap().clone().ok()
                    } else {
                        None
                    }
                })
                .map(|data| {
                    data.and_then(|d| {
                        if d.values.is_empty() {
                            return None;
                        }

                        match d.operation.as_str() {
                            "sum" => Some(d.values.iter().sum()),
                            "product" => Some(d.values.iter().product()),
                            _ => None,
                        }
                    })
                })
                .end()
        };

        let test_inputs = [
            r#"{"operation": "sum", "values": [1, 2, 3, 4]}"#,
            r#"{"operation": "product", "values": [2, 3, 4]}"#,
            "invalid json",
            r#"{"operation": "invalid", "values": [1, 2]}"#,
            r#"{"operation": "sum", "values": []}"#,
        ];

        let results: Vec<_> = test_inputs.iter().map(|s| process_complex_data(s)).collect();

        assert_eq!(results[0], Some(10));
        assert_eq!(results[1], Some(24));
        assert_eq!(results[2], None);
        assert_eq!(results[3], None);
        assert_eq!(results[4], None);
    }

    #[test]
    fn should_handle_validation_errors_gracefully_in_data_processing() {
        #[derive(Debug)]
        struct UserObject {
            email: String,
            name: String,
            age: i32,
        }

        #[derive(Debug)]
        #[allow(dead_code)]
        struct ValidationResult {
            valid: bool,
            error: Option<String>,
            name: String,
            email: String,
            age: i32,
        }

        let validate_and_process = |user: Option<UserObject>| -> ValidationResult {
            let parse_result = umt_safe_execute_mut(|| {
                user.ok_or_else(|| "Invalid user object".to_string())
            });

            if parse_result.is_error() {
                return ValidationResult {
                    valid: false,
                    error: Some("Invalid object".to_string()),
                    name: "Unknown".to_string(),
                    email: String::new(),
                    age: 0,
                };
            }

            let user_data = parse_result.value().unwrap();
            match user_data {
                Ok(u) => {
                    let email_validator = umt_validate_email_validator(None, None);
                    let email_valid = (email_validator.validate)(&u.email);

                    let age_result = umt_safe_execute_mut(|| {
                        if u.age < 0 || u.age > 150 {
                            panic!("Invalid age");
                        }
                        u.age
                    });

                    let error = if !email_valid {
                        Some("Invalid email".to_string())
                    } else if age_result.is_error() {
                        Some("Invalid age".to_string())
                    } else {
                        None
                    };

                    ValidationResult {
                        valid: email_valid && age_result.is_success(),
                        error,
                        name: u.name.clone(),
                        email: u.email.clone(),
                        age: if age_result.is_success() {
                            *age_result.value().unwrap()
                        } else {
                            0
                        },
                    }
                }
                Err(_) => ValidationResult {
                    valid: false,
                    error: Some("Invalid object".to_string()),
                    name: "Unknown".to_string(),
                    email: String::new(),
                    age: 0,
                },
            }
        };

        let test_data = vec![
            Some(UserObject {
                email: "alice@example.com".to_string(),
                name: "Alice".to_string(),
                age: 30,
            }),
            Some(UserObject {
                email: "invalid-email".to_string(),
                name: "Bob".to_string(),
                age: 25,
            }),
            Some(UserObject {
                email: "charlie@test.com".to_string(),
                name: "Charlie".to_string(),
                age: -1,
            }),
            None,
            Some(UserObject {
                email: "diana@example.com".to_string(),
                name: "Diana".to_string(),
                age: -5,
            }),
        ];

        let results: Vec<_> = test_data.into_iter().map(validate_and_process).collect();

        assert!(results[0].valid);
        assert!(!results[1].valid);
        assert_eq!(results[1].error, Some("Invalid email".to_string()));
        assert!(!results[2].valid);
        assert_eq!(results[2].error, Some("Invalid age".to_string()));
        assert!(!results[3].valid);
        assert_eq!(results[3].error, Some("Invalid object".to_string()));
        assert!(!results[4].valid);
        assert_eq!(results[4].error, Some("Invalid age".to_string()));
    }

    #[test]
    fn should_provide_error_recovery_mechanisms() {
        let process_with_retry = |max_retries: usize| -> (bool, Option<String>, usize) {
            let mut attempts = 0;
            let call_count = std::cell::RefCell::new(0);

            while attempts <= max_retries {
                let _count = *call_count.borrow();
                let result = umt_safe_execute_mut(|| -> String {
                    *call_count.borrow_mut() += 1;
                    let current = *call_count.borrow();
                    if current < 3 {
                        panic!("Attempt {} failed", current);
                    }
                    "15".to_string()
                });

                if result.is_success() {
                    return (
                        true,
                        result.value().cloned(),
                        attempts + 1,
                    );
                }

                attempts += 1;
            }

            (false, None, attempts)
        };

        let result = process_with_retry(3);

        assert!(result.0);
        assert_eq!(result.1, Some("15".to_string()));
        assert_eq!(result.2, 3);
    }
}
