import { average } from "./average";

/**
 * Calculate the Pearson correlation coefficient between two arrays
 * @param x - First array of numbers
 * @param y - Second array of numbers
 * @returns Correlation coefficient (-1 to 1)
 * @example
 * correlationCoefficient([1, 2, 3, 4, 5], [2, 4, 6, 8, 10]); // 1 (perfect positive correlation)
 * correlationCoefficient([1, 2, 3, 4, 5], [5, 4, 3, 2, 1]); // -1 (perfect negative correlation)
 * correlationCoefficient([1, 2, 3, 4, 5], [1, 1, 1, 1, 1]); // 0 (no correlation)
 */
export const correlationCoefficient = (x: number[], y: number[]): number => {
  if (x.length === 0) {
    return Number.NaN;
  }

  if (x.length === 1) {
    return Number.NaN;
  }

  const meanX = average(x);
  const meanY = average(y);

  let numerator = 0;
  let sumSquaredX = 0;
  let sumSquaredY = 0;

  for (const [index, element] of x.entries()) {
    const deltaX = element - meanX;
    const deltaY = y[index] - meanY;

    numerator += deltaX * deltaY;
    sumSquaredX += deltaX * deltaX;
    sumSquaredY += deltaY * deltaY;
  }

  const denominator = Math.sqrt(sumSquaredX * sumSquaredY);

  if (denominator === 0) {
    return Number.NaN;
  }

  return numerator / denominator;
};
