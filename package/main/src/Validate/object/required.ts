/**
 * Object validation - required
 * Returns a new object validator where every key is unwrapped from any
 * `optional()` layer, mirroring `Required<T>` at the type level so consumers
 * see all properties as required again.
 */

import { object, type ObjectShape, type ObjectValidator } from "./core";
import type { OptionalValidator } from "./optional";

export type UnwrapOptional<V> =
  V extends OptionalValidator<infer Inner, infer R> ? (value: Inner) => R : V;

export type RequiredShape<T extends ObjectShape> = {
  [K in keyof T]: UnwrapOptional<T[K]> extends ObjectShape[string]
    ? UnwrapOptional<T[K]>
    : T[K];
};

/**
 * Strips `optional()` wrappers from every property validator of `validator`.
 * Validators that are not wrapped in `optional()` are kept as-is.
 * @template T - Original object shape
 * @param {ObjectValidator<T>} validator - Source object validator
 * @param {string} [message] - Custom error message for the new validator
 * @returns {ObjectValidator<RequiredShape<T>>} - New validator where every optional layer is removed
 */
export const required = <T extends ObjectShape>(
  validator: ObjectValidator<T>,
  message?: string,
): ObjectValidator<RequiredShape<T>> => {
  const sourceShape = validator.shape;
  const nextShape = {} as ObjectShape;
  for (const key of Object.keys(sourceShape)) {
    const current = sourceShape[key];
    const optionalLike = current as unknown as {
      isOptional?: boolean;
      inner?: ObjectShape[string];
    };
    nextShape[key] =
      optionalLike.isOptional === true &&
      typeof optionalLike.inner === "function"
        ? optionalLike.inner
        : current;
  }
  return object(nextShape, message) as unknown as ObjectValidator<
    RequiredShape<T>
  >;
};
