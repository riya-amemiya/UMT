/**
 * 小数かどうかを判定する
 * @param {unknown} x
 * @param {boolean} [loose=true] - 文字列も対象にするかどうか
 * @returns boolean
 * @example isDouble(0.1); // true
 * isDouble("0.1"); // true
 * isDouble("0.1", false); // false
 */

function isDouble(x: unknown): x is number | string;
function isDouble(x: unknown, loose: false): x is number;
function isDouble(x: unknown, loose = true): x is number {
  if (loose) {
    return (
      // biome-ignore lint/suspicious/noGlobalIsFinite: <explanation>
      isFinite(x as number) &&
      !Number.isNaN(x) &&
      Number.isFinite(Number(x)) &&
      !Number.isInteger(Number(x))
    );
  }
  return Number.isFinite(x) && !Number.isInteger(x);
}

export { isDouble };