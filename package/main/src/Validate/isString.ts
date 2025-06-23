/**
 * Determines if the value is a string
 * @param  {unknown} value - Value to check
 * @returns boolean - True if the value is a string, false otherwise
 * @example isString("test"); // true
 * isString(123); // false
 */
const isString = (value: unknown): value is string => {
  return typeof value === "string";
};

export { isString };
