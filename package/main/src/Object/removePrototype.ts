/**
 * Creates a new object with the same properties as the given object, but with the prototype polluting properties removed.
 * ("__proto__", "constructor", "prototype" are excluded from the shallow copy)
 *
 * @param object - The object to remove the prototype polluting properties from.
 * @returns A new object with the prototype polluting properties removed.
 *
 * @example
 * ```typescript
 * const obj = JSON.parse('{"__proto__":{"polluted":true},"a":1,"b":2}');
 * const safeObj = removePrototype(obj);
 * // safeObj is { a: 1, b: 2 } and "__proto__" is removed
 * ```
 */
export const removePrototype = <T extends Record<string, unknown>>(
  object: T,
): Omit<T, "__proto__" | "constructor" | "prototype"> => {
  const result: Record<string, unknown> = Object.create(null);
  const keys = Object.keys(object);

  for (const key of keys) {
    if (key !== "__proto__" && key !== "constructor" && key !== "prototype") {
      result[key] = object[key];
    }
  }

  return result as Omit<T, "__proto__" | "constructor" | "prototype">;
};
