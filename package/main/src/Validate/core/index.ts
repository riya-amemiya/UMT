/**
 * Core validation module
 * Provides the base validation functionality used by all other validation modules
 */

import type {
  Types,
  ValidateCoreReturnType,
  ValidateReturnType,
} from "@/Validate/type";

const EMPTY_OPTION: ValidateReturnType<unknown>[] = [];

/**
 * Creates a validator function that checks type and additional validation rules
 * @template T - The type of value to validate
 * @param {Types<T>} type - The expected type of the value
 * @returns {Function} - A validator function that accepts a value, options, and message
 */
export const core = <T>(type: Types<T>) => {
  const success: ValidateCoreReturnType<T> = {
    validate: true,
    message: "",
    type,
  };
  return <O extends ValidateReturnType<T>[]>(
    value: T,
    option: O = EMPTY_OPTION as unknown as O,
    message?: string,
  ): ValidateCoreReturnType<T> => {
    if (typeof value !== type) {
      return {
        validate: false,
        message: message ?? "",
        type,
      };
    }
    const length = option.length;
    for (let index = 0; index < length; index++) {
      const rule = option[index];
      if (!rule.validate(value)) {
        return {
          validate: false,
          message: rule.message ?? "",
          type,
        };
      }
    }
    return success;
  };
};
