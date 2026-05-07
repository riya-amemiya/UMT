import { isArray } from "@/Validate/isArray";
import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Creates an array validator that validates every element with a single validator
 * @template T - Element type validated by the wrapped validator
 * @param {Function} validator - Validator applied to each element (e.g. an object validator)
 * @param {string} [message] - Custom error message for array type validation
 * @returns {Function} - Validator function for arrays whose elements satisfy the given validator
 */
export const arrayOf = <T>(
  validator: (value: T) => ValidateCoreReturnType<T>,
  message?: string,
) => {
  return (values: T[]): ValidateCoreReturnType<T[]> => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message ?? "",
        type: values,
      };
    }
    for (const value of values) {
      const result = validator(value);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type: values,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: values,
    };
  };
};
