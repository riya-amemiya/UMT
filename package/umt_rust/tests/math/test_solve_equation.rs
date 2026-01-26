use umt_rust::math::umt_solve_equation;

#[test]
fn test_solve_equation_2x2_case1() {
    // System:
    // x + y = 4
    // x + 2y = 10
    let result = umt_solve_equation(
        &mut vec![vec![1.0, 1.0], vec![1.0, 2.0]],
        &mut vec![4.0, 10.0],
    );
    assert_eq!(result, vec![-2.0, 6.0]);
}

#[test]
fn test_solve_equation_2x2_case2() {
    // System:
    // x + 6y = 33
    // x + y = 8
    let result = umt_solve_equation(
        &mut vec![vec![1.0, 6.0], vec![1.0, 1.0]],
        &mut vec![33.0, 8.0],
    );
    assert_eq!(result, vec![3.0, 5.0]);
}

#[test]
fn test_solve_equation_3x3() {
    // System:
    // 5x - 4y + 6z = 8
    // 7x - 6y + 10z = 14
    // 4x + 9y + 7z = 74
    let result = umt_solve_equation(
        &mut vec![
            vec![5.0, -4.0, 6.0],
            vec![7.0, -6.0, 10.0],
            vec![4.0, 9.0, 7.0],
        ],
        &mut vec![8.0, 14.0, 74.0],
    );
    assert_eq!(result, vec![2.0, 5.0, 3.0]);
}

use umt_rust::math::*;

#[test]
fn test_solve_equation_basic() {
    // x + y = 4
    // x + 2y = 10
    let result = umt_solve_equation(
        &mut vec![vec![1.0, 1.0], vec![1.0, 2.0]],
        &mut vec![4.0, 10.0],
    );
    assert_eq!(result, vec![-2.0, 6.0]);
}

#[test]
fn test_solve_equation_single() {
    // 2x = 6
    let result = umt_solve_equation(&mut vec![vec![2.0]], &mut vec![6.0]);
    assert_eq!(result, vec![3.0]);
}
