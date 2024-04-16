import { range } from "@/Array/range";

/**
 * 条件式を満たす数値の配列を返す
 * @param start 開始数値
 * @param end 終了数値
 * @param conditionalExpression 条件式
 * @returns 条件式を満たす数値の配列
 * @example rangeAdvance(1, 10, (number) => number % 2 === 0); // [2, 4, 6, 8]
 */
const rangeAdvance = (
  start: number,
  end?: number,
  conditionalExpression?: (number_: number) => boolean,
): number[] => {
  if (conditionalExpression) {
    const array: number[] = [];
    for (let index = start; index < (end as number); index++) {
      conditionalExpression(index) && array.push(index);
    }
    return array;
  }
  return range(start, end);
};
export { rangeAdvance };
