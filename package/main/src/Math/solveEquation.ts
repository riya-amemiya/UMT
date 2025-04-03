import { addition } from "./addition";
import { division } from "./division";
import { multiplication } from "./multiplication";
import { roundOf } from "./roundOf";
import { subtract } from "./subtract";
/**
 * Solves a system of linear equations using Gaussian elimination
 * @param {number[][]} coefficients Matrix of coefficients
 * @param {number[]} constants Vector of constants
 * @returns {number[]} Solution vector
 * @example
 * // Solves the system:
 * // x + y = 4
 * // x + 2y = 10
 * solveEquation([
 *   [1, 1],
 *   [1, 2]
 * ], [4, 10]); // returns [-2, 6]
 * @description
 * Uses Gaussian elimination with partial pivoting to solve a system of linear equations.
 * The solution is rounded to 1 decimal place.
 */
export const solveEquation = (
  coefficients: number[][],
  constants: number[],
): number[] => {
  const n = constants.length;
  for (let index = 0; index < n; index++) {
    // Find the max element in the column (partial pivoting)
    let maxElement = Math.abs(coefficients[index][index]);
    let maxRow = index;
    for (let row = index + 1; row < n; row++) {
      if (Math.abs(coefficients[row][index]) > maxElement) {
        maxElement = Math.abs(coefficients[row][index]);
        maxRow = row;
      }
    }

    // Swap the row with the max element to the top of the matrix
    for (let col = index; col < n; col++) {
      const temporary = coefficients[maxRow][col];
      coefficients[maxRow][col] = coefficients[index][col];
      coefficients[index][col] = temporary;
    }

    const temporary = constants[maxRow];
    constants[maxRow] = constants[index];
    constants[index] = temporary;

    // Perform elimination on the rows below
    for (let row = index + 1; row < n; row++) {
      const factor = division(
        coefficients[row][index],
        coefficients[index][index],
      );

      constants[row] = subtract(
        constants[row],
        multiplication(factor, constants[index]),
      );

      for (let col = index; col < n; col++) {
        coefficients[row][col] = subtract(
          coefficients[row][col],
          multiplication(factor, coefficients[index][col]),
        );
      }
    }
  }
  const solution: number[] = [];

  // Back substitute to find the solution
  for (let row = n - 1; row >= 0; row--) {
    let sum = 0;
    for (let col = row + 1; col < n; col++) {
      sum = addition(
        sum,
        multiplication(coefficients[row][col], solution[n - col - 1]),
      );
    }

    solution.push(
      division(subtract(constants[row], sum), coefficients[row][row]),
    );
  }

  // Return the solution vector with values rounded to 1 decimal place
  return solution.reverse().map((value) => roundOf(value, 1));
};
