/**
 * Array validation core module
 * Provides functionality to validate arrays with type-specific validation rules
 */

import { isArray } from "@/Validate/isArray";
import { isNotEmpty } from "@/Validate/isNotEmpty";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

/**
 * Creates an array validator with type-specific validation rules
 * @template A - Type of array elements (string | number | boolean)
 * @template O - Object containing validation functions for each type
 * @param {O} option - Validation functions for each type
 * @param {string} [message] - Custom error message for array type validation
 * @returns {Function} - Validator function for arrays
 */
export const array = <
  A extends string | number | boolean,
  O extends {
    [P in Types<A>]: (
      value: ValidateType<P>,
    ) => ValidateCoreReturnType<ValidateType<P>>;
  } = {
    [P in Types<A>]: (
      value: ValidateType<P>,
    ) => ValidateCoreReturnType<ValidateType<P>>;
  },
>(
  option: O = {} as O,
  message?: string,
) => {
  return (values: A[]): ValidateCoreReturnType<A[]> => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message || "",
        type: values,
      };
    }
    if (isNotEmpty(option)) {
      for (const value of values) {
        const validator = option[typeof value as Types<A>];
        if (!validator?.(value as never).validate) {
          return {
            validate: false,
            message: validator?.(value as never).message || "",
            type: values,
          };
        }
      }
    }
    return {
      validate: true,
      message: "",
      type: values,
    };
  };
};
