use umt_plugin_wasm::solve_equation;

#[test]
fn test_solve_equation() {
    // fn solve_equation(coefficients_ptr: *const f64, constants_ptr: *const f64, rows: usize, cols: usize) -> *mut f64
    assert_eq!(solve_equation(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]], &[1.0, 2.0, 3.0]), &[1.0, 2.0, 3.0], 3,3);
}