/**
 * Determines if the value is a decimal number
 * @param {unknown} x - Value to check
 * @param {boolean} [loose=true] - Whether to include string representations of decimal numbers
 * @returns boolean - True if the value is a decimal number, false otherwise
 * @example isDouble(0.1); // true
 * isDouble("0.1"); // true
 * isDouble("0.1", false); // false
 */

const isDouble = <T extends boolean = true>(
  x: unknown,
  loose: T = true as T,
): x is T extends true ? number | string : number => {
  if (loose) {
    return (
      // biome-ignore lint/suspicious/noGlobalIsFinite: ignore
      isFinite(x as number) &&
      !Number.isNaN(x) &&
      Number.isFinite(Number(x)) &&
      !Number.isInteger(Number(x))
    );
  }
  return Number.isFinite(x) && !Number.isInteger(x);
};

export { isDouble };
