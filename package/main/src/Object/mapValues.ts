/**
 * Creates an object with the same keys as the given object, but
 * with values transformed by the provided function.
 *
 * @param object - The source object.
 * @param function_ - The function invoked per value. Receives (value, key).
 * @returns A new object with transformed values.
 *
 * @example
 * ```typescript
 * mapValues({ a: 1, b: 2 }, (value) => value * 2);
 * // { a: 2, b: 4 }
 * ```
 */
export const mapValues = <T extends Record<string, unknown>, R>(
  object: T,
  function_: (value: T[keyof T], key: string) => R,
): Record<keyof T, R> => {
  const result = {} as Record<string, R>;
  const keys = Object.keys(object);
  const length = keys.length;
  let index = 0;

  while (index < length) {
    const key = keys[index];
    result[key] = function_(object[key] as T[keyof T], key);
    index += 1;
  }

  return result as Record<keyof T, R>;
};
