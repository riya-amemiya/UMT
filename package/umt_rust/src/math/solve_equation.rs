use super::umt_round_of;

/// Solves a system of linear equations using Gaussian elimination.
///
/// Uses Gaussian elimination with partial pivoting to solve a system of linear equations.
/// The solution is rounded to 1 decimal place.
///
/// # Arguments
///
/// * `coefficients` - Matrix of coefficients.
/// * `constants` - Vector of constants.
///
/// # Returns
///
/// Solution vector.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_solve_equation;
///
/// // Solves the system:
/// // x + y = 4
/// // x + 2y = 10
/// let result = umt_solve_equation(
///     &mut vec![vec![1.0, 1.0], vec![1.0, 2.0]],
///     &mut vec![4.0, 10.0]
/// );
/// assert_eq!(result, vec![-2.0, 6.0]);
/// ```
#[allow(clippy::ptr_arg, clippy::needless_range_loop)]
pub fn umt_solve_equation(coefficients: &mut Vec<Vec<f64>>, constants: &mut Vec<f64>) -> Vec<f64> {
    let n = constants.len();

    for index in 0..n {
        // Find the max element in the column (partial pivoting)
        let mut max_element = coefficients[index][index].abs();
        let mut max_row = index;

        for row in (index + 1)..n {
            if coefficients[row][index].abs() > max_element {
                max_element = coefficients[row][index].abs();
                max_row = row;
            }
        }

        // Swap the row with the max element to the top of the matrix
        for col in index..n {
            let temp = coefficients[max_row][col];
            coefficients[max_row][col] = coefficients[index][col];
            coefficients[index][col] = temp;
        }

        constants.swap(max_row, index);

        // Perform elimination on the rows below
        for row in (index + 1)..n {
            let factor = coefficients[row][index] / coefficients[index][index];

            constants[row] -= factor * constants[index];

            for col in index..n {
                coefficients[row][col] -= factor * coefficients[index][col];
            }
        }
    }

    let mut solution: Vec<f64> = vec![0.0; n];

    // Back substitute to find the solution
    for row in (0..n).rev() {
        let mut sum = 0.0;
        for col in (row + 1)..n {
            sum += coefficients[row][col] * solution[col];
        }

        solution[row] = (constants[row] - sum) / coefficients[row][row];
    }

    // Return the solution vector with values rounded to 1 decimal place
    solution.iter().map(|&v| umt_round_of(v, 1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_solve_equation_3x3() {
        // 2x + y + z = 9
        // x + 2y + z = 8
        // x + y + 2z = 7
        let result = umt_solve_equation(
            &mut vec![
                vec![2.0, 1.0, 1.0],
                vec![1.0, 2.0, 1.0],
                vec![1.0, 1.0, 2.0],
            ],
            &mut vec![9.0, 8.0, 7.0],
        );
        assert_eq!(result, vec![3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_solve_equation_single() {
        // 2x = 6
        let result = umt_solve_equation(&mut vec![vec![2.0]], &mut vec![6.0]);
        assert_eq!(result, vec![3.0]);
    }
}
