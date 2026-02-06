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
 * Extract key-value pairs common to all input objects.
 *
 * A key-value pair is considered common when the key exists in every object
 * and the value is strictly equal (===) across all objects.
 * When all values for a key are plain objects, the function recurses to find
 * the common subset. If the recursive result is empty, the key is excluded.
 *
 * @template T - Type of the input objects.
 * @param {T} object - The first object.
 * @param {...Record<string, unknown>[]} objects - Additional objects to compare.
 * @returns {Partial<T>} Object containing only the key-value pairs shared by all inputs.
 *
 * @example
 * ```typescript
 * getObjectsCommon({ a: 1, b: 2 }, { a: 1, c: 3 });
 * // { a: 1 }
 *
 * getObjectsCommon({ a: { b: 1, c: 2 }, d: 3 }, { a: { b: 1, d: 4 }, d: 3 });
 * // { a: { b: 1 }, d: 3 }
 * ```
 */
export const getObjectsCommon = <T extends Record<string, unknown>>(
  object: T,
  ...objects: Record<string, unknown>[]
): Partial<T> => {
  if (objects.length === 0) {
    return { ...object };
  }

  const result = {} as Partial<T>;

  for (const [key, value] of Object.entries(object)) {
    let isCommon = true;
    let allPlainObjects = isPlainObject(value);

    for (const other of objects) {
      if (!Object.hasOwn(other, key)) {
        isCommon = false;
        break;
      }

      if (!isPlainObject(other[key])) {
        allPlainObjects = false;
      }

      if (!allPlainObjects && other[key] !== value) {
        isCommon = false;
        break;
      }
    }

    if (!isCommon) {
      continue;
    }

    if (allPlainObjects) {
      const nested = getObjectsCommon(
        value as Record<string, unknown>,
        ...objects.map(
          (other) => other[key] as Record<string, unknown>,
        ),
      );

      if (Object.keys(nested).length > 0) {
        (result as Record<string, unknown>)[key] = nested;
      }
    } else {
      (result as Record<string, unknown>)[key] = value;
    }
  }

  return result;
};
