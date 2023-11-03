import { average } from "./average";
/**
 * 標準偏差
 * @param  {number[]} values
 * @returns number
 * @example standardDeviation([1, 2, 3]); // 0.816496580927726
 */
export const standardDeviation = (values: number[]) => {
  const avg = average(values);
  const squareDiffs = values.map((value) => {
    const diff = value - avg;
    const sqrDiff = diff * diff;
    return sqrDiff;
  });
  const avgSquareDiff = average(squareDiffs);
  const standardDeviation_ = Math.sqrt(avgSquareDiff);
  return standardDeviation_;
};
