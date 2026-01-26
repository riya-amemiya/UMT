use umt_rust::function::{
    umt_curry0, umt_curry1, umt_curry2, umt_curry3, umt_curry4, umt_curry5, umt_curry6,
};

#[test]
fn test_curry_function_with_0_arguments() {
    let func = || "hello";
    let curried_func = umt_curry0(func);
    assert_eq!(curried_func(), "hello");
}

#[test]
fn test_curry_function_with_1_argument() {
    let func = |a: i32| a * 2;
    let curried_func = umt_curry1(func);
    assert_eq!(curried_func(5), 10);
}

#[test]
fn test_curry_function_with_2_arguments() {
    let func = |a: i32, b: i32| a + b;
    let curried_func = umt_curry2(func);
    assert_eq!(curried_func(2)(3), 5);
}

#[test]
fn test_curry_function_with_3_arguments() {
    let func = |a: i32, b: i32, c: i32| a * b + c;
    let curried_func = umt_curry3(func);
    assert_eq!(curried_func(2)(3)(4), 10);
}

#[test]
fn test_curry_function_with_4_arguments() {
    let func = |a: i32, b: i32, c: i32, d: i32| a + b + c + d;
    let curried_func = umt_curry4(func);
    assert_eq!(curried_func(1)(2)(3)(4), 10);
}

#[test]
fn test_curry_function_with_5_arguments() {
    let func = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
    let curried_func = umt_curry5(func);
    assert_eq!(curried_func(1)(2)(3)(4)(5), 15);
}

#[test]
fn test_curry_function_with_6_arguments() {
    let func = |a: i32, b: i32, c: i32, d: i32, e: i32, f: i32| a + b + c + d + e + f;
    let curried_func = umt_curry6(func);
    assert_eq!(curried_func(1)(2)(3)(4)(5)(6), 21);
}

#[test]
fn test_curry_with_different_argument_types() {
    let func = |a: i32, b: &str, c: bool| format!("{}{}{}", a, b, c);
    let curried_func = umt_curry3(func);
    assert_eq!(curried_func(1)("hello")(true), "1hellotrue");
}

#[test]
fn test_curry_with_object_like_types() {
    // Using a struct to simulate object-like behavior
    #[derive(Clone)]
    struct Point {
        x: i32,
    }

    let func = |obj: Point, arr: Vec<i32>| obj.x + arr.iter().sum::<i32>();
    let curried_func = umt_curry2(func);
    assert_eq!(curried_func(Point { x: 5 })(vec![1, 2, 3]), 11);
}

#[test]
fn test_curry_with_function_arguments() {
    let func = |f: fn(i32) -> i32, x: i32| f(x);
    let curried_func = umt_curry2(func);
    let double: fn(i32) -> i32 = |x| x * 2;
    assert_eq!(curried_func(double)(5), 10);
}

#[test]
fn test_curry_partial_application_reuse() {
    // Test that curried functions can be reused with different arguments
    let add = |a: i32, b: i32| a + b;
    let curried_add = umt_curry2(add);
    let add_five = curried_add(5);

    assert_eq!(add_five(1), 6);
    assert_eq!(add_five(10), 15);
    assert_eq!(add_five(100), 105);
}

#[test]
fn test_curry_with_closures_capturing_context() {
    let multiplier = 2;
    let multiply = move |a: i32, b: i32| a * b * multiplier;
    let curried_multiply = umt_curry2(multiply);
    assert_eq!(curried_multiply(3)(4), 24);
}

#[test]
fn test_curry_with_string_types() {
    let concat = |a: String, b: String| format!("{}{}", a, b);
    let curried = umt_curry2(concat);
    assert_eq!(
        curried("Hello, ".to_string())("World!".to_string()),
        "Hello, World!"
    );
}

#[test]
fn test_curry_with_float_types() {
    let multiply = |a: f64, b: f64| a * b;
    let curried = umt_curry2(multiply);
    let result = curried(2.5)(4.0);
    assert!((result - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_curry_composition() {
    // Test using curried functions in composition-like patterns
    let add = |a: i32, b: i32| a + b;
    let multiply = |a: i32, b: i32| a * b;

    let curried_add = umt_curry2(add);
    let curried_multiply = umt_curry2(multiply);

    let add_one = curried_add(1);
    let double = curried_multiply(2);

    let value = 5;
    let result = double(add_one(value));
    assert_eq!(result, 12); // (5 + 1) * 2 = 12
}

#[test]
fn test_curry_with_negative_numbers() {
    let subtract = |a: i32, b: i32| a - b;
    let curried = umt_curry2(subtract);
    assert_eq!(curried(-5)(3), -8);
    assert_eq!(curried(3)(-5), 8);
}

#[test]
fn test_curry_with_zero() {
    let add = |a: i32, b: i32| a + b;
    let curried = umt_curry2(add);
    assert_eq!(curried(0)(0), 0);
    assert_eq!(curried(0)(5), 5);
    assert_eq!(curried(5)(0), 5);
}

#[test]
fn test_curry_chain_evaluation() {
    // Verify that the curried function evaluates correctly at each step
    let sum3 = |a: i32, b: i32, c: i32| a + b + c;
    let curried = umt_curry3(sum3);

    // First partial application
    let step1 = curried(10);
    // Second partial application
    let step2 = step1(20);
    // Final evaluation
    let result = step2(30);

    assert_eq!(result, 60);
}

#[test]
fn test_curry_with_boolean_result() {
    let greater_than = |a: i32, b: i32| a > b;
    let curried = umt_curry2(greater_than);

    assert!(curried(5)(3));
    assert!(!curried(3)(5));
    assert!(!curried(5)(5));
}

#[test]
fn test_curry_with_optional_result() {
    let safe_divide = |a: f64, b: f64| {
        if b == 0.0 { None } else { Some(a / b) }
    };

    let curried = umt_curry2(safe_divide);

    assert_eq!(curried(10.0)(2.0), Some(5.0));
    assert_eq!(curried(10.0)(0.0), None);
}

use umt_rust::function::*;

#[test]
fn test_curry0() {
    let hello = || "hello";
    let curried = umt_curry0(hello);
    assert_eq!(curried(), "hello");
}

#[test]
fn test_curry1() {
    let double = |a: i32| a * 2;
    let curried = umt_curry1(double);
    assert_eq!(curried(5), 10);
}

#[test]
fn test_curry2() {
    let add = |a: i32, b: i32| a + b;
    let curried = umt_curry2(add);
    assert_eq!(curried(2)(3), 5);
}

#[test]
fn test_curry3() {
    let multiply_add = |a: i32, b: i32, c: i32| a * b + c;
    let curried = umt_curry3(multiply_add);
    assert_eq!(curried(2)(3)(4), 10);
}

#[test]
fn test_curry4() {
    let sum4 = |a: i32, b: i32, c: i32, d: i32| a + b + c + d;
    let curried = umt_curry4(sum4);
    assert_eq!(curried(1)(2)(3)(4), 10);
}

#[test]
fn test_curry5() {
    let sum5 = |a: i32, b: i32, c: i32, d: i32, e: i32| a + b + c + d + e;
    let curried = umt_curry5(sum5);
    assert_eq!(curried(1)(2)(3)(4)(5), 15);
}

#[test]
fn test_curry6() {
    let sum6 = |a: i32, b: i32, c: i32, d: i32, e: i32, g: i32| a + b + c + d + e + g;
    let curried = umt_curry6(sum6);
    assert_eq!(curried(1)(2)(3)(4)(5)(6), 21);
}
