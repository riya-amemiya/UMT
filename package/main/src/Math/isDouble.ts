/**
 * @param {unknown} x
 * @param {boolean} [loose=true] - 文字列も対象にするかどうか
 * @returns boolean
 */

function isDouble(x: unknown): x is number | string;
function isDouble(x: unknown, loose: false): x is number;
function isDouble(x: unknown, loose = true): x is number {
  if (loose) {
    return (
      // biome-ignore lint/nursery/noGlobalIsFinite: <explanation>
      isFinite(x as number) &&
      !Number.isNaN(x) &&
      Number.isFinite(Number(x)) &&
      !Number.isInteger(Number(x))
    );
  } else {
    return Number.isFinite(x);
  }
}

export { isDouble };
