/**
 * 数字かどうか
 * @param  {unknown} x
 * @param  {boolean} loose 文字列も対象にするかどうか
 * @returns boolean
 */

function isNumber(x: unknown): x is number | string;
function isNumber(x: unknown, loose: false): x is number;
function isNumber(x: unknown, loose = true): x is number {
  return x !== null && typeof x !== "boolean" && loose
    ? // rome-ignore lint/nursery/noGlobalIsFinite: <explanation>
      isFinite(x as number)
    : Number.isFinite(x);
}

export { isNumber };
