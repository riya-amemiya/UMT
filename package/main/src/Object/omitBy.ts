/**
 * Creates a new object containing only entries for which the predicate returns false.
 *
 * @template T - Type of the source object
 * @param {T} object - Source object
 * @param {(value: T[keyof T], key: keyof T) => boolean} predicate - Rejection predicate
 * @returns {Partial<T>} A new object with the remaining entries
 * @example
 * omitBy({ a: 1, b: undefined, c: 3 }, (v) => v === undefined); // { a: 1, c: 3 }
 */
export const omitBy = <T extends Record<string, unknown>>(
  object: T,
  predicate: (value: T[keyof T], key: keyof T) => boolean,
): Partial<T> => {
  const result: Partial<T> = {};
  for (const key of Object.keys(object) as (keyof T)[]) {
    const value = object[key];
    if (!predicate(value, key)) {
      result[key] = value;
    }
  }
  return result;
};
