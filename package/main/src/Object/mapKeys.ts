/**
 * Creates an object with the same values as the given object, but
 * with keys transformed by the provided function.
 *
 * @param object - The source object.
 * @param function_ - The function invoked per key. Receives (value, key).
 * @returns A new object with transformed keys.
 *
 * @example
 * ```typescript
 * mapKeys({ a: 1, b: 2 }, (_value, key) => key.toUpperCase());
 * // { A: 1, B: 2 }
 * ```
 */
export const mapKeys = <T extends Record<string, unknown>>(
  object: T,
  function_: (value: T[keyof T], key: string) => string,
): Record<string, T[keyof T]> => {
  const result: Record<string, T[keyof T]> = {};
  const keys = Object.keys(object);
  const length = keys.length;
  let index = 0;

  while (index < length) {
    const key = keys[index];
    // Skip prototype pollution keys from source
    if (key === "__proto__" || key === "constructor" || key === "prototype") {
      index += 1;
      continue;
    }
    const value = object[key] as T[keyof T];
    const newKey = function_(value, key);
    // Prevent prototype pollution via transformed keys
    if (
      newKey === "__proto__" ||
      newKey === "constructor" ||
      newKey === "prototype"
    ) {
      index += 1;
      continue;
    }
    result[newKey] = value;
    index += 1;
  }

  return result;
};
