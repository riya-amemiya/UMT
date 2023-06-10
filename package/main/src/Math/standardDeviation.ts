import { average } from "./average";
/**
 * 標準偏差
 * @param  {number[]} values
 */
export const standardDeviation = (values: number[]) => {
  const avg = average(values);
  const squareDiffs = values.map((value) => {
    const diff = value - avg;
    const sqrDiff = diff * diff;
    return sqrDiff;
  });
  const avgSquareDiff = average(squareDiffs);
  const stdDev = Math.sqrt(avgSquareDiff);
  return stdDev;
};
