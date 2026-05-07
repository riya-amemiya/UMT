/**
 * Object validation - pick
 * Returns a new object validator that only checks the specified keys of the
 * source validator. Mirrors `Pick<T, K>` at the type level so consumers like
 * `union()`, `intersection()`, and `SchemaToInterface` see the narrowed shape.
 *
 * The function is exported as `pick_` because the top-level `Object` module
 * already exposes a `pick` runtime helper.
 */

import { object, type ObjectShape, type ObjectValidator } from "./core";

/**
 * Picks the given keys from an existing object validator and returns a new
 * object validator covering only those keys.
 * @template T - Original object shape
 * @template K - Tuple of keys to pick from the original shape
 * @param {ObjectValidator<T>} validator - Source object validator
 * @param {K} keys - Keys to retain on the new validator
 * @param {string} [message] - Custom error message for the picked validator
 * @returns {ObjectValidator<Pick<T, K[number]>>} - New validator scoped to picked keys
 */
export const pick_ = <T extends ObjectShape, K extends readonly (keyof T)[]>(
  validator: ObjectValidator<T>,
  keys: K,
  message?: string,
): ObjectValidator<Pick<T, K[number]>> => {
  const sourceShape = validator.shape;
  const nextShape = {} as Pick<T, K[number]>;
  for (const key of keys) {
    nextShape[key] = sourceShape[key];
  }
  return object(nextShape, message);
};
