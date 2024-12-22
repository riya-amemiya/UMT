/**
 * 連続した数値の配列を生成する
 * @param start 開始数値
 * @param end 終了数値 (省略した場合は0からstartまでの配列を生成)
 * @param step ステップ数 (省略した場合は1)
 * @returns 連続した数値の配列
 * @example range(5); // [0, 1, 2, 3, 4]
 * @example range(2, 10, 2); // [2, 4, 6, 8]
 */
const range = (start: number, end?: number, step = 1) => {
  const array: number[] = [];
  if (end) {
    for (let index = start; index < end; index += step) {
      array.push(index);
    }
  } else {
    for (let index = 0; index < start; index += step) {
      array.push(index);
    }
  }
  return array;
};
export { range };
