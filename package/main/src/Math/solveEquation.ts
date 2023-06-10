import { addition } from "./addition";
import { division } from "./division";
import { multiplication } from "./multiplication";
import { roundOf } from "./roundOf";
import { subtract } from "./subtract";
/**
 * Solve a system of linear equations using Gauss-Jordan elimination
 * @param {Equation} equation - The system of linear equations
 * @returns {number[]} The solution to the system of linear equations
 * @example
 * x + y = 4
 * x + 2y = 10
 * solveEquation(
 * {
 * 	coefficients: [
 * 		[1, 1],
 * 		[1, 2],
 * 	],
 * 	constants: [4, 10],
 * },
 * ); // [-2, 6]
 */
export const solveEquation = (
  coefficients: number[][],
  constants: number[],
): number[] => {
  const n = constants.length;
  for (let i = 0; i < n; i++) {
    // Find the max element in the column
    let maxElement = Math.abs(coefficients[i][i]);
    let maxRow = i;
    for (let j = i + 1; j < n; j++) {
      if (Math.abs(coefficients[j][i]) > maxElement) {
        maxElement = Math.abs(coefficients[j][i]);
        maxRow = j;
      }
    }

    // Swap the row with the max element to the top of the matrix
    for (let j = i; j < n; j++) {
      const tmp = coefficients[maxRow][j];
      coefficients[maxRow][j] = coefficients[i][j];
      coefficients[i][j] = tmp;
    }

    const tmp = constants[maxRow];
    constants[maxRow] = constants[i];
    constants[i] = tmp;

    // Perform elimination on the rows below
    for (let j = i + 1; j < n; j++) {
      // const factor = coefficients[j][i] / coefficients[i][i];
      const factor = division(coefficients[j][i], coefficients[i][i]);
      // constants[j] -= factor * constants[i];
      constants[j] = subtract(
        constants[j],
        multiplication(factor, constants[i]),
      );
      for (let k = i; k < n; k++) {
        // coefficients[j][k] -= factor * coefficients[i][k];
        coefficients[j][k] = subtract(
          coefficients[j][k],
          multiplication(factor, coefficients[i][k]),
        );
      }
    }
  }
  const solution: number[] = [];

  // Back substitute to find the solution
  for (let i = n - 1; i >= 0; i--) {
    let sum = 0;
    for (let j = i + 1; j < n; j++) {
      // sum += coefficients[i][j] * solution[n - j - 1];
      sum = addition(
        sum,
        multiplication(
          coefficients[i][j],
          solution[subtract(subtract(n, j), 1)],
        ),
      );
    }
    // solution.push((constants[i] - sum) / coefficients[i][i]);
    solution.push(division(subtract(constants[i], sum), coefficients[i][i]));
  }
  return solution.reverse().map((x) => roundOf(x, 1));
};
