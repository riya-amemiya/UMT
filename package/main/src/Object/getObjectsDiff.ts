import { isPlainObject } from "@/Object/isPlainObject";

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
 * @remarks
 * **Prototype pollution warning:** This function does not filter out
 * prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
 * If processing user-controlled input, sanitize with the appropriate
 * `removePrototype*` helper before calling this function:
 * - `removePrototype` — shallow sanitization of a single object
 * - `removePrototypeDeep` — recursive sanitization of a single object (for deeply nested data)
 * - `removePrototypeMap` — shallow sanitization of an array of objects
 * - `removePrototypeMapDeep` — recursive sanitization of an array of objects (for deeply nested data)
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
  for (const object_ of allObjects) {
    for (const key of Object.keys(object_)) {
      allKeys.add(key);
    }
  }

  const result = {} as Partial<T>;

  for (const key of allKeys) {
    const values: unknown[] = [];
    const presentIn: number[] = [];

    for (const [index, currentObject] of allObjects.entries()) {
      if (Object.hasOwn(currentObject, key)) {
        values.push(currentObject[key]);
        presentIn.push(index);
      }
    }

    if (values.length === 1) {
      (result as Record<string, unknown>)[key] = values[0];
      continue;
    }

    const allPlain = values.every((value) => isPlainObject(value));

    if (allPlain) {
      const nested = getObjectsDiff(values[0] as T, ...values.slice(1));

      if (Object.keys(nested).length > 0) {
        (result as Record<string, unknown>)[key] = nested;
      }
      continue;
    }

    let lastUniqueValue: unknown;
    let hasUnique = false;

    // Count occurrences in a single pass using a Map — O(n) instead of O(n²).
    // Map keys use SameValueZero which matches === for all values except NaN,
    // an acceptable trade-off for object-diff use cases.
    const valueCounts = new Map<unknown, number>();
    for (const value of values) {
      valueCounts.set(value, (valueCounts.get(value) ?? 0) + 1);
    }
    for (const [value, count] of valueCounts) {
      if (count === 1) {
        lastUniqueValue = value;
        hasUnique = true;
      }
    }

    if (hasUnique) {
      (result as Record<string, unknown>)[key] = lastUniqueValue;
    }
  }

  return result;
};
