import { addition } from "./addition";
import { division } from "./division";
import { multiplication } from "./multiplication";
import { roundOf } from "./roundOf";
import { subtract } from "./subtract";
/**
 * 連立一次方程式を解く
 * @param {Equation} equation - 連立一次方程式
 * @returns {number[]} 解
 * @example
 * // x + y = 4
 * // x + 2y = 10
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
  for (let index = 0; index < n; index++) {
    // Find the max element in the column
    let maxElement = Math.abs(coefficients[index][index]);
    let maxRow = index;
    for (let index_ = index + 1; index_ < n; index_++) {
      if (Math.abs(coefficients[index_][index]) > maxElement) {
        maxElement = Math.abs(coefficients[index_][index]);
        maxRow = index_;
      }
    }

    // Swap the row with the max element to the top of the matrix
    for (let index_ = index; index_ < n; index_++) {
      const temporary_ = coefficients[maxRow][index_];
      coefficients[maxRow][index_] = coefficients[index][index_];
      coefficients[index][index_] = temporary_;
    }

    const temporary = constants[maxRow];
    constants[maxRow] = constants[index];
    constants[index] = temporary;

    // Perform elimination on the rows below
    for (let index_ = index + 1; index_ < n; index_++) {
      // const factor = coefficients[j][i] / coefficients[i][i];
      const factor = division(
        coefficients[index_][index],
        coefficients[index][index],
      );
      // constants[j] -= factor * constants[i];
      constants[index_] = subtract(
        constants[index_],
        multiplication(factor, constants[index]),
      );
      for (let k = index; k < n; k++) {
        // coefficients[j][k] -= factor * coefficients[i][k];
        coefficients[index_][k] = subtract(
          coefficients[index_][k],
          multiplication(factor, coefficients[index][k]),
        );
      }
    }
  }
  const solution: number[] = [];

  // Back substitute to find the solution
  for (let index = n - 1; index >= 0; index--) {
    let sum = 0;
    for (let index_ = index + 1; index_ < n; index_++) {
      // sum += coefficients[i][j] * solution[n - j - 1];
      sum = addition(
        sum,
        multiplication(
          coefficients[index][index_],
          solution[subtract(subtract(n, index_), 1)],
        ),
      );
    }
    // solution.push((constants[i] - sum) / coefficients[i][i]);
    solution.push(
      division(subtract(constants[index], sum), coefficients[index][index]),
    );
  }
  return solution.reverse().map((x) => roundOf(x, 1));
};
