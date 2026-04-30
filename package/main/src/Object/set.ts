import { pathSegments } from "./pathSegments";

/**
 * Sets a value at a deeply nested path on an object, mutating it.
 * Creates intermediate objects as needed.
 *
 * @template T - Type of the source object
 * @param {T} object - Target object (mutated)
 * @param {string | string[]} path - Dot-separated string or path segments array
 * @param {unknown} value - Value to assign at the path
 * @returns {T} The same object reference, for chaining
 *
 * @remarks
 * **Prototype pollution warning:** This function does not filter out
 * prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
 * Sanitize user-controlled paths before calling.
 *
 * @example
 * set({}, "a.b.c", 1); // { a: { b: { c: 1 } } }
 */
export const set = <T extends object>(
  object: T,
  path: string | string[],
  value: unknown,
): T => {
  const segments = pathSegments(path);
  if (segments.length === 0) {
    return object;
  }
  let current: Record<string, unknown> = object as Record<string, unknown>;
  for (let index = 0; index < segments.length - 1; index += 1) {
    const key = segments[index];
    const next = current[key];
    if (next == null || typeof next !== "object") {
      current[key] = {};
    }
    current = current[key] as Record<string, unknown>;
  }
  const lastIndex = segments.length - 1;
  current[segments[lastIndex]] = value;
  return object;
};
