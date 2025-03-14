/**
 * Determines if an object has a specified path
 * @param {T} object - Object to check
 * @param {string | string[]} path - Path to check
 * @returns {boolean} Returns true if path exists, false otherwise
 * @example has({ a: { b: 1 } }, "a.b"); // true
 * has({ a: { b: 1 } }, ["a", "b"]); // true
 * has({ a: { b: 1 } }, "a.c"); // false
 */
export const has = <T extends { [key: string]: unknown }>(
  object: T,
  path: string | string[],
): boolean => {
  const localPath = typeof path === "string" ? path.split(".") : path;
  let current = { ...object };
  for (const key of localPath) {
    if (
      current == null ||
      !Object.prototype.hasOwnProperty.call(current, key)
    ) {
      return false;
    }
    current = current[key] as T;
  }
  return true;
};
