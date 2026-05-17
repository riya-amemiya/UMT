/**
 * Set validation core module
 * Provides validation for `Set` instances. The validator can optionally
 * delegate to a per-element validator, mirroring how `arrayOf()` validates
 * each element of an array.
 *
 * The function is exported as `set_` because the top-level `Object` module
 * already exposes a `set` runtime helper.
 */

import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

export type SetExtractValidatedType<V> = V extends (value: never) => {
  type: infer T;
}
  ? ValidateType<T>
  : never;

/**
 * Creates a Set validator. When a per-element validator is supplied, every
 * element of the set must satisfy it; iteration short-circuits at the first
 * failure and surfaces the failing message.
 * @template IV - Validator for set elements
 * @param {IV} [itemValidator] - Validator applied to every element
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function for Set instances
 */
export const set_ = <
  // biome-ignore lint/suspicious/noExplicitAny: validator inputs vary
  IV extends (value: any) => ValidateCoreReturnType<unknown> = (
    value: unknown,
  ) => ValidateCoreReturnType<unknown>,
  T = SetExtractValidatedType<IV>,
>(
  itemValidator?: IV,
  message?: string,
): ((value: Set<T>) => ValidateCoreReturnType<Set<T>>) &
  StandardSchemaV1<Set<T>, Set<T>> => {
  const setValidator = (value: Set<T>): ValidateCoreReturnType<Set<T>> => {
    if (!(value instanceof Set)) {
      return {
        validate: false,
        message: message ?? "",
        type: value,
      };
    }
    if (itemValidator) {
      for (const item of value) {
        const result = (
          itemValidator as (v: unknown) => {
            validate: boolean;
            message: string;
          }
        )(item);
        if (!result.validate) {
          return {
            validate: false,
            message: result.message,
            type: value,
          };
        }
      }
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  };
  return attachStandard<Set<T>, Set<T>, typeof setValidator>(setValidator);
};
