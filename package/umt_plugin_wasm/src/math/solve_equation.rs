
pub fn umt_solve_equation(mut coefficients: Vec<Vec<f64>>, mut constants: Vec<f64>) -> Vec<f64> {
	let n = constants.len();
	for i in 0..n {
		// Find the max element in the column
		let mut max_element = coefficients[i][i].abs();
		let mut max_row = i;
		for j in i + 1..n {
			if coefficients[j][i].abs() > max_element {
				max_element = coefficients[j][i].abs();
				max_row = j;
			}
		}

		// Swap the row with the max element to the top of the matrix
		for j in i..n {
			let tmp = coefficients[max_row][j];
			coefficients[max_row][j] = coefficients[i][j];
			coefficients[i][j] = tmp;
		}

		let tmp = constants[max_row];
		constants[max_row] = constants[i];
		constants[i] = tmp;

		// Perform elimination on the rows below
		for j in i + 1..n {
			let factor = coefficients[j][i] / coefficients[i][i];
			constants[j] -= factor * constants[i];
			for k in i..n {
				coefficients[j][k] -= factor * coefficients[i][k];
			}
		}
	}
	let mut solution = Vec::new();

	// Back substitute to find the solution
	for i in (0..n).rev() {
		let mut sum = 0.0;
		for j in i + 1..n {
			sum += coefficients[i][j] * solution[n - j - 1];
		}
		solution.push((constants[i] - sum) / coefficients[i][i]);
	}
	solution.reverse();
	solution
}