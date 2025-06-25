/**
 * Determines if the value represents a number
 * @param  {unknown} number - Value to check
 * @param  {boolean} loose - Whether to include string representations of numbers (default: true)
 * @returns boolean - True if the value represents a number, false otherwise
 * @example isNumber(0.1); // true
 * isNumber("0.1"); // true
 * isNumber("0.1", false); // false
 */
const isNumber = <T extends boolean>(
  number: unknown,
  loose: T = true as T,
): number is T extends true ? number | string : number => {
  if (Array.isArray(number)) {
    return false;
  }
  if (typeof number === "object") {
    return false;
  }
  return number !== null && typeof number !== "boolean" && loose
    ? // biome-ignore lint/suspicious/noGlobalIsFinite: ignore
      isFinite(number as number)
    : Number.isFinite(number as number);
};

export { isNumber };
