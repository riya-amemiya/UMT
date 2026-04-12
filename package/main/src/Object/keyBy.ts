type PropertyName = string | number | symbol;
type IterateeFunction<T> = (value: T) => PropertyName;
type Iteratee<T> = IterateeFunction<T> | keyof T;

/**
 * Creates an object composed of keys generated from the results of running each element of collection through iteratee
 * @param collection The collection to iterate over
 * @param iteratee The iteratee function or property name to generate the key
 *
 * @remarks
 * **Prototype pollution warning:** This function does not filter out
 * prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
 * If processing user-controlled input, sanitize with the appropriate
 * `removePrototype*` helper before calling this function:
 * - `removePrototype` — shallow sanitization of a single object
 * - `removePrototypeDeep` — recursive sanitization of a single object (for deeply nested data)
 * - `removePrototypeMap` — shallow sanitization of an array of objects
 * - `removePrototypeMapDeep` — recursive sanitization of an array of objects (for deeply nested data)
 */
export function keyBy<T>(
  collection: T[] | Record<PropertyName, T>,
  iteratee?: Iteratee<T>,
): Record<PropertyName, T> {
  const getKey = normalizeIteratee(iteratee);
  const result: Record<PropertyName, T> = {};

  if (Array.isArray(collection)) {
    for (const value of collection) {
      const key = getKey(value);
      result[key] = value;
    }
    return result;
  }

  for (const value of Object.values(collection)) {
    const key = getKey(value);
    result[key] = value;
  }
  return result;
}

/**
 * Normalizes the iteratee function
 */
function normalizeIteratee<T>(iteratee?: Iteratee<T>): IterateeFunction<T> {
  if (!iteratee) {
    return (value) => value as unknown as PropertyName;
  }
  if (typeof iteratee === "function") {
    return iteratee;
  }
  return (object: T) => object[iteratee] as PropertyName;
}
