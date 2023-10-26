import { range } from "..";

/**
 * 条件式を満たす数値の配列を返す
 * @param start 開始数値
 * @param end 終了数値
 * @param conditionalExpression 条件式
 * @returns
 */
function rangeAdvance(
  start: number,
  end?: number,
  conditionalExpression?: (num: number) => boolean,
): number[] {
  if (conditionalExpression) {
    const arr = [];
    for (let i = start; i < (end as number); i++) {
      conditionalExpression(i) && arr.push(i);
    }
    return arr;
  }
  return range(start, end);
}
export { rangeAdvance };
