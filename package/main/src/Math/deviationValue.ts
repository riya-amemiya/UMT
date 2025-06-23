/**
 * Calculate standard score (deviation value)
 * @param  {number} value - Current value
 * @param  {number} averageValue - Mean value
 * @param  {number} standardDeviationValue - Standard deviation
 * @returns {number} Standard score (where 50 is average, each standard deviation is worth 10 points)
 * @example deviationValue(10, 5, 2); // 75
 */
export const deviationValue = (
  value: number,
  averageValue: number,
  standardDeviationValue: number,
): number => {
  return ((value - averageValue) / standardDeviationValue) * 10 + 50;
};
