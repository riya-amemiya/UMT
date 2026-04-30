/**
 * Reads a deeply nested property by string or string-array path.
 * Returns the default value if any segment is missing.
 *
 * @template T - Expected return type
 * @param {unknown} object - Source object
 * @param {string | string[]} path - Dot-separated string or path segments array
 * @param {T} [defaultValue] - Returned when the path resolves to undefined or is missing
 * @returns {T | undefined} The resolved value, or defaultValue when missing
 * @example
 * get({ a: { b: { c: 1 } } }, "a.b.c"); // 1
 * get({ a: { b: 1 } }, ["a", "x"], 0); // 0
 */
import { pathSegments } from "./pathSegments";

export const get = <T>(
  object: unknown,
  path: string | string[],
  defaultValue?: T,
): T | undefined => {
  const segments = pathSegments(path);
  let current: unknown = object;
  for (const key of segments) {
    if (current == null) {
      return defaultValue;
    }
    current = (current as Record<string, unknown>)[key];
  }
  return (current === undefined ? defaultValue : current) as T | undefined;
};
