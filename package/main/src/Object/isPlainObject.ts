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
  if (value === null || typeof value !== "object") {
    return false;
  }

  const prototype = Object.getPrototypeOf(value);
  if (prototype === null) {
    return true;
  }

  if (prototype === Object.prototype) {
    return true;
  }

  // Handle objects created via Object.create(plainObject)
  // Check constructor on the prototype to avoid own-property shadowing
  const protoConstructor = prototype.constructor;
  return typeof protoConstructor === "function" && protoConstructor === Object;
};
