/**
 * Checks if a value is a plain object
 */
const isPlainObject = (value: unknown): value is Record<string, unknown> => {
  return (
    value !== null &&
    typeof value === "object" &&
    value.constructor === Object &&
    Object.prototype.toString.call(value) === "[object Object]"
  );
};

/**
 * Extract key-value pairs that appear in exactly one input object.
 *
 * A key-value pair is considered "shared" if the same key with the same value (===)
 * exists in two or more objects. Only pairs unique to a single object are returned.
 * When all values for a key are plain objects, the function recurses to find
 * the diff subset. If the recursive result is empty, the key is excluded.
 * When multiple unique pairs share the same key (different values),
 * the last value wins.
 *
 * @template T - Type of the input objects.
 * @param {T} object - The first object.
 * @param {...Record<string, unknown>[]} objects - Additional objects to compare.
 * @returns {Partial<T>} Object containing only key-value pairs unique to one input.
 *
 * @example
 * ```typescript
 * getObjectsDiff({ a: 1, b: 2 }, { b: 2, c: 3 });
 * // { a: 1, c: 3 }
 *
 * getObjectsDiff({ a: { b: 1, c: 2 }, d: 3 }, { a: { b: 1, d: 4 }, d: 3 });
 * // { a: { c: 2, d: 4 } }
 * ```
 */
export const getObjectsDiff = <T extends Record<string, unknown>>(
  object: T,
  ...objects: Record<string, unknown>[]
): Partial<T> => {
  const allObjects = [object as Record<string, unknown>, ...objects];

  if (allObjects.length === 1) {
    return { ...object };
  }

  const allKeys = new Set<string>();
  for (const obj of allObjects) {
    for (const key of Object.keys(obj)) {
      allKeys.add(key);
    }
  }

  const result = {} as Partial<T>;

  for (const key of allKeys) {
    const values: unknown[] = [];
    const presentIn: number[] = [];

    for (let i = 0; i < allObjects.length; i++) {
      if (Object.hasOwn(allObjects[i], key)) {
        values.push(allObjects[i][key]);
        presentIn.push(i);
      }
    }

    if (values.length === 1) {
      (result as Record<string, unknown>)[key] = values[0];
      continue;
    }

    const allPlain = values.every(isPlainObject);

    if (allPlain) {
      const nested = getObjectsDiff(
        values[0] as Record<string, unknown> as T,
        ...(values.slice(1) as Record<string, unknown>[]),
      );

      if (Object.keys(nested).length > 0) {
        (result as Record<string, unknown>)[key] = nested;
      }
      continue;
    }

    let lastUniqueValue: unknown = undefined;
    let hasUnique = false;

    for (let i = 0; i < values.length; i++) {
      let count = 0;
      for (let j = 0; j < values.length; j++) {
        if (values[i] === values[j]) {
          count++;
        }
      }
      if (count === 1) {
        lastUniqueValue = values[i];
        hasUnique = true;
      }
    }

    if (hasUnique) {
      (result as Record<string, unknown>)[key] = lastUniqueValue;
    }
  }

  return result;
};
