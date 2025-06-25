import type { PickDeep } from "$/object/pickDeep";
import type { PickDeepKey } from "$/object/pickDeepKey";

/**
 * Creates a new object by deeply selecting properties from the source object based on specified keys.
 *
 * @template T - Type of the source object.
 * @template K - Type of property keys to select. Must be a subset of PickDeepKey<T>.
 * @param {T} object - The source object to extract properties from.
 * @param {...K[]} keys - Property keys to extract. Can use dot notation for nested properties.
 * @returns {PickDeep<T, K>} A new object containing only the specified properties.
 *
 * @example
 * ```typescript
 * const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
 * const picked = pickDeep(obj, 'a.b.c', 'f');
 * // picked will be { a: { b: { c: 1 } }, f: 4 }
 * ```
 */
export const pickDeep = <
  T extends object,
  const K extends readonly PickDeepKey<T>[],
>(
  object: T,
  ...keys: K
): PickDeep<T, K> => {
  // biome-ignore lint/suspicious/noExplicitAny: ignore
  const result: any = {};

  for (const key of keys) {
    const parts = (key as string).split(".");
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    let current: any = { ...object };
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    let target: any = result;

    for (const [index, part] of parts.entries()) {
      if (current && typeof current === "object" && part in current) {
        if (index === parts.length - 1) {
          target[part] = current[part];
        } else {
          target[part] = target[part] || {};
          current = current[part];
          target = target[part];
        }
      }
    }
  }

  return result;
};
