/**
 * Determines if a value is NaN
 * @param value - Value to check
 * @param loose - If true, includes string values in NaN check (default: false)
 * @returns boolean - True if the value is NaN, false otherwise
 * @example
 * isValueNaN(1); // false
 * isValueNaN("NaN"); // false
 * isValueNaN("NaN", true); // true
 * isValueNaN(parseInt("not a number")); // true
 */
export const isValueNaN = (value: unknown, loose = false): boolean => {
  // biome-ignore lint/suspicious/noGlobalIsNan: <explanation>
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
