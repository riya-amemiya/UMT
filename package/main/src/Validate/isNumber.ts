/**
 * 数字を表すかどうかを判定する
 * @param  {unknown} number
 * @param  {boolean} loose 文字列も対象にするかどうか (default: true)
 * @returns boolean
 * @example isNumber(0.1); // true
 * isNumber("0.1"); // true
 * isNumber("0.1", false); // false
 */
const isNumber = <T extends boolean>(
  number: unknown,
  loose: T = true as T,
): number is T extends true ? number | string : number => {
  return number !== null && typeof number !== "boolean" && loose
    ? // biome-ignore lint/suspicious/noGlobalIsFinite: <explanation>
      isFinite(number as number)
    : Number.isFinite(number as number);
};

export { isNumber };
