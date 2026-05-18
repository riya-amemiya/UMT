/**
 * Object validation - omit
 * Returns a new object validator that drops the specified keys from the
 * source validator. Mirrors `Omit<T, K>` at the type level so consumers
 * like `union()`, `intersection()`, and `SchemaToInterface` see the
 * narrowed shape.
 *
 * The function is exported as `omit_` because the top-level `Object` module
 * already exposes an `omit` runtime helper.
 */

import {
  type InferObject,
  object,
  type ObjectShape,
  type ObjectValidator,
  type StandardSchemaV1,
} from "./core";

/**
 * Removes the given keys from an existing object validator and returns a new
 * object validator covering the remaining keys.
 * @template T - Original object shape
 * @template K - Tuple of keys to omit from the original shape
 * @param {ObjectValidator<T>} validator - Source object validator
 * @param {K} keys - Keys to drop from the new validator
 * @param {string} [message] - Custom error message for the omitted validator
 * @returns {ObjectValidator<Omit<T, K[number]>>} - New validator without omitted keys
 */
export const omit_ = <T extends ObjectShape, K extends readonly (keyof T)[]>(
  validator: ObjectValidator<T>,
  keys: K,
  message?: string,
): ObjectValidator<Omit<T, K[number]>> &
  StandardSchemaV1<
    InferObject<Omit<T, K[number]>>,
    InferObject<Omit<T, K[number]>>
  > => {
  const sourceShape = validator.shape;
  const omittedKeys = new Set<keyof T>(keys);
  const nextShape = {} as ObjectShape;
  for (const key in sourceShape) {
    if (!omittedKeys.has(key)) {
      nextShape[key] = sourceShape[key];
    }
  }
  return object(nextShape, message) as unknown as ObjectValidator<
    Omit<T, K[number]>
  > &
    StandardSchemaV1<
      InferObject<Omit<T, K[number]>>,
      InferObject<Omit<T, K[number]>>
    >;
};
