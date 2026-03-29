/**
 * Determines if an object has a specified path
 * @param {T} object - Object to check
 * @param {string | string[]} path - Path to check
 * @returns {boolean} Returns true if path exists, false otherwise
 * @example has({ a: { b: 1 } }, "a.b"); // true
 * has({ a: { b: 1 } }, ["a", "b"]); // true
 * has({ a: { b: 1 } }, "a.c"); // false
 */
// Security: Keys that must be blocked to prevent prototype pollution
// when traversing objects with user-controlled paths.
const DANGEROUS_KEYS = new Set(["__proto__", "constructor", "prototype"]);

export const has = <T extends { [key: string]: unknown }>(
  object: T,
  path: string | string[],
): boolean => {
  const localPath = typeof path === "string" ? path.split(".") : path;
  let current = { ...object };
  for (const key of localPath) {
    // Security: Block prototype pollution keys to prevent prototype chain traversal
    if (DANGEROUS_KEYS.has(key)) {
      return false;
    }
    if (current == null || !Object.hasOwn(current, key)) {
      return false;
    }
    current = current[key] as T;
  }
  return true;
};
