/**
 * Determines if an object has a specified path
 * @param {T} object - Object to check
 * @param {string | string[]} path - Path to check
 * @returns {boolean} Returns true if path exists, false otherwise
 * @example has({ a: { b: 1 } }, "a.b"); // true
 * has({ a: { b: 1 } }, ["a", "b"]); // true
 * has({ a: { b: 1 } }, "a.c"); // false
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
import { pathSegments } from "./pathSegments";

export const has = <T extends { [key: string]: unknown }>(
  object: T,
  path: string | string[],
): boolean => {
  const segments = pathSegments(path);
  let current: unknown = object;
  for (const key of segments) {
    if (current == null || !Object.hasOwn(current, key)) {
      return false;
    }
    current = (current as Record<string, unknown>)[key];
  }
  return true;
};
