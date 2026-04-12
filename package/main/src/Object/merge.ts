import type { UnionToIntersection } from "$/logic/unionToIntersection";

/**
 * Merges multiple objects into a single object (shallow merge)
 * @param target - The target object to merge into
 * @param sources - The source objects to merge from
 * @returns The merged object
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
export const merge = <
  T extends Record<string, unknown>,
  U extends Record<string, unknown>[],
>(
  target: T,
  ...sources: U
): T & UnionToIntersection<U[number]> => {
  const result: Record<string, unknown> = {};
  for (const object of [target, ...sources]) {
    for (const key of Object.keys(object)) {
      result[key] = object[key];
    }
  }
  return result as T & UnionToIntersection<U[number]>;
};
