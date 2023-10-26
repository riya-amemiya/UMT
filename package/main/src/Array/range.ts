/**
 * 連続した数値の配列を生成する
 * @param start 開始数値
 * @param end 終了数値 (省略した場合は0からstartまでの配列を生成)
 * @returns 連続した数値の配列
 */
function range(start: number, end?: number) {
  const arr = [];
  if (end) {
    for (let i = start; i < end; i++) {
      arr.push(i);
    }
  } else {
    for (let i = 0; i < start; i++) {
      arr.push(i);
    }
  }
  return arr;
}
export { range };
