/**
 * 数字かどうか
 * @param  {unknown} x
 * @param  {boolean} loose 文字列も対象にするかどうか (default: true)
 * @returns boolean
 */

function isNumber<T extends boolean>(
  number: unknown,
  loose: T = true as T,
): number is T extends true ? number | string : number {
  return number !== null && typeof number !== "boolean" && loose
    ? // biome-ignore lint/nursery/noGlobalIsFinite: <explanation>
      isFinite(number as number)
    : Number.isFinite(number as number);
}

export { isNumber };
