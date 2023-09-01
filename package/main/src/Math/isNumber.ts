/**
 * 数字かどうか
 * @param  {any} x
 * @param  {boolean} loose 文字列も対象にするかどうか
 * @returns boolean
 */

// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const isNumber = (x: any, loose = true) => {
  return x !== null && typeof x !== "boolean" && loose
    ? // rome-ignore lint/nursery/noGlobalIsFinite: <explanation>
      isFinite(x)
    : Number.isFinite(x);
};
