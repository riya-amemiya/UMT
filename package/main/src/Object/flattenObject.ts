import { isPlainObject } from "./isPlainObject";

/**
 * Recursively flattens a nested plain object into a single-level object
 * keyed by joined paths. Arrays and non-plain objects are kept as-is.
 *
 * @param {Record<string, unknown>} object - Source nested object
 * @param {string} [separator="."] - Separator used to join path segments
 * @returns {Record<string, unknown>} A flat object
 * @example
 * flattenObject({ a: { b: { c: 1 } }, d: 2 }); // { "a.b.c": 1, d: 2 }
 */
export const flattenObject = <T extends Record<string, unknown>>(
  object: T,
  separator = ".",
): Record<string, unknown> => {
  const result: Record<string, unknown> = {};

  const walk = (current: Record<string, unknown>, prefix: string): void => {
    for (const key of Object.keys(current)) {
      const value = current[key];
      const nextKey = prefix ? `${prefix}${separator}${key}` : key;
      if (isPlainObject(value) && Object.keys(value).length > 0) {
        walk(value, nextKey);
      } else {
        result[nextKey] = value;
      }
    }
  };

  walk(object, "");
  return result;
};
