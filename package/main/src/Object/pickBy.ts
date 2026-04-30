/**
 * Creates a new object containing only entries for which the predicate returns true.
 *
 * @template T - Type of the source object
 * @param {T} object - Source object
 * @param {(value: T[keyof T], key: keyof T) => boolean} predicate - Selector predicate
 * @returns {Partial<T>} A new object with the filtered entries
 * @example
 * pickBy({ a: 1, b: 2, c: 3 }, (v) => v > 1); // { b: 2, c: 3 }
 */
export const pickBy = <T extends Record<string, unknown>>(
  object: T,
  predicate: (value: T[keyof T], key: keyof T) => boolean,
): Partial<T> => {
  const result: Partial<T> = {};
  for (const key of Object.keys(object) as (keyof T)[]) {
    const value = object[key];
    if (predicate(value, key)) {
      result[key] = value;
    }
  }
  return result;
};
