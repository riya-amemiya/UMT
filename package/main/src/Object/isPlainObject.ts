/**
 * Checks if a value is a plain object (created by Object constructor
 * or with a null prototype).
 *
 * @param value - The value to check.
 * @returns True if the value is a plain object, false otherwise.
 *
 * @example
 * ```typescript
 * isPlainObject({});          // true
 * isPlainObject({ a: 1 });    // true
 * isPlainObject(new Map());   // false
 * isPlainObject(null);        // false
 * ```
 */
export const isPlainObject = (
  value: unknown,
): value is Record<string, unknown> => {
  return (
    value !== null &&
    typeof value === "object" &&
    value.constructor === Object &&
    Object.prototype.toString.call(value) === "[object Object]"
  );
};
