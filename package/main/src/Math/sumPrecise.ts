/**
 * Computes the sum of an array of numbers using the Neumaier
 * summation algorithm for improved floating-point precision.
 *
 * @param numbers - The array of numbers to sum.
 * @returns The precise sum of all numbers.
 *
 * @example
 * ```typescript
 * sumPrecise([0.1, 0.2, 0.3]);    // 0.6
 * sumPrecise([1e20, 1, -1e20]);    // 1
 * ```
 */
export const sumPrecise = (numbers: number[]): number => {
  let sum = 0;
  let compensation = 0;

  for (const number_ of numbers) {
    const t = sum + number_;
    compensation +=
      Math.abs(sum) >= Math.abs(number_)
        ? sum - t + number_
        : number_ - t + sum;
    sum = t;
  }

  return sum + compensation;
};
