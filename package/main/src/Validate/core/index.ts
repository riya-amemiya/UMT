/**
 * Core validation module
 * Provides the base validation functionality used by all other validation modules
 */

import type {
  Types,
  ValidateCoreReturnType,
  ValidateReturnType,
} from "@/Validate/type";

/**
 * Creates a validator function that checks type and additional validation rules
 * @template T - The type of value to validate
 * @param {Types<T>} type - The expected type of the value
 * @returns {Function} - A validator function that accepts a value, options, and message
 */

export const core =
  <T>(type: Types<T>) =>
  <O extends ValidateReturnType<T>[]>(
    value: T,
    option: O = [] as unknown as O,
    message?: string,
  ): ValidateCoreReturnType<T> => {
    if (typeof value !== type) {
      return {
        validate: false,
        message: message || "",
        type,
      };
    }
    for (const validate of option) {
      if (!validate.validate(value)) {
        return {
          validate: false,
          message: validate.message || "",
          type,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type,
    };
  };
