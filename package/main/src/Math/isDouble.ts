/**
 * @param {any} x
 * @param {boolean} [loose=true] - 文字列も対象にするかどうか
 * @returns boolean
 */

// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const isDouble = (x: any, loose = true) => {
  if (loose) {
    return (
      Number.isFinite(x) &&
      !Number.isNaN(x) &&
      Number.isFinite(Number(x)) &&
      !Number.isInteger(Number(x))
    );
  } else {
    return x !== null && typeof x !== "boolean" && Number.isFinite(x);
  }
};
