import { set } from "./set";

/**
 * Reconstructs a nested object from a flat path-keyed object.
 *
 * @param {Record<string, unknown>} flat - Flat input keyed by joined paths
 * @param {string} [separator="."] - Separator used in path keys
 * @returns {Record<string, unknown>} A nested object
 *
 * @remarks
 * **Prototype pollution warning:** Internally uses `set`, which does not
 * filter out prototype-polluting keys. Sanitize user-controlled keys.
 *
 * @example
 * unflattenObject({ "a.b.c": 1, d: 2 }); // { a: { b: { c: 1 } }, d: 2 }
 */
export const unflattenObject = (
  flat: Record<string, unknown>,
  separator = ".",
): Record<string, unknown> => {
  const result: Record<string, unknown> = {};
  for (const key of Object.keys(flat)) {
    set(result, key.split(separator), flat[key]);
  }
  return result;
};
