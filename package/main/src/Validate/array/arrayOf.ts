/**
 * Array validation module for arrays whose elements are validated by a single validator
 * Useful for expressing arrays of objects, arrays of unions, and other structured element types
 */

import { isArray } from "@/Validate/isArray";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

// Extract the validated element type by reading the validator's `type` field
// (and applying `ValidateType` to map type tags like "string" back to the
// runtime type). Reading the field directly preserves literal unions and
// object shapes that would otherwise be collapsed by `Types<T>`.
type ExtractValidatedType<V> = V extends (value: never) => { type: infer T }
  ? ValidateType<T>
  : never;

/**
 * Creates an array validator that validates every element with a single validator
 * @template V - Validator function type for array elements
 * @param {V} validator - Validator applied to each element (e.g. an object validator)
 * @param {string} [message] - Custom error message for array type validation
 * @returns {Function} - Validator function for arrays whose elements satisfy the given validator
 */
export const arrayOf = <
  V extends (value: never) => ValidateCoreReturnType<unknown>,
>(
  validator: V,
  message?: string,
) => {
  type Element = ExtractValidatedType<V>;
  return (values: Element[]): ValidateCoreReturnType<Element[]> => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message ?? "",
        type: values as unknown as Types<Element[]>,
      };
    }
    for (const value of values) {
      const result = (
        validator as unknown as (v: Element) => ValidateCoreReturnType<Element>
      )(value);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type: values as unknown as Types<Element[]>,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: values as unknown as Types<Element[]>,
    };
  };
};
