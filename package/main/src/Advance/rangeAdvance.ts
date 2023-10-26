/**
 * 条件式を満たす数値の配列を返す
 * @param start 開始数値
 * @param end 終了数値
 * @param conditionalExpression 条件式
 * @returns
 */
export const rangeAdvance = (
  start: number,
  end?: number,
  conditionalExpression?: (num: number) => boolean,
) => {
  const arr = [];
  if (!end) {
    for (let i = 0; i <= start; i++) {
      conditionalExpression
        ? conditionalExpression(i) && arr.push(i)
        : arr.push(i);
    }
    return arr;
  }
  for (let i = start; i <= end; i++) {
    conditionalExpression
      ? conditionalExpression(i) && arr.push(i)
      : arr.push(i);
  }
  return arr;
};
