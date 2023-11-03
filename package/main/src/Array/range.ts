/**
 * 連続した数値の配列を生成する
 * @param start 開始数値
 * @param end 終了数値 (省略した場合は0からstartまでの配列を生成)
 * @returns 連続した数値の配列
 * @example range(5); // [0, 1, 2, 3, 4]
 */
function range(start: number, end?: number) {
  const array = [];
  if (end) {
    for (let index = start; index < end; index++) {
      array.push(index);
    }
  } else {
    for (let index = 0; index < start; index++) {
      array.push(index);
    }
  }
  return array;
}
export { range };
