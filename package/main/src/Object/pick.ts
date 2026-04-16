/**
 * Creates a new object with only the specified properties from the source object.
 *
 * @template T - Type of the source object.
 * @template K - Type of the keys to extract (subset of T's keys).
 * @param {T} object - The source object to extract properties from.
 * @param {...K[]} keys - The property keys to extract.
 * @returns {Pick<T, K>} A new object containing only the specified properties.
 *
 * @example
 * ```typescript
 * const user = { id: 1, name: 'Alice', age: 30 };
 * const selected = pick(user, 'id', 'name'); // { id: 1, name: 'Alice' }
 * ```
 */
export const pick = <T extends object, K extends keyof T>(
  object: T,
  ...keys: K[]
): Pick<T, K> => {
  const result = {} as Pick<T, K>;
  for (const key of keys) {
    // Security: reject prototype-polluting keys. If an attacker bypasses
    // TypeScript's compile-time key constraint (via cast or untrusted
    // input), assigning to result["__proto__"] would invoke the
    // Object.prototype __proto__ setter and replace result's prototype
    // with the attacker-supplied value, tainting every subsequent
    // property access on the returned object.
    if (key === "__proto__" || key === "constructor" || key === "prototype") {
      continue;
    }
    result[key] = object[key];
  }
  return result;
};
