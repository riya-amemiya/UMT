/**
 * Null validation core module
 * Provides the base validation functionality for null values
 */

import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Creates a null validator
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is null
 */
export const nullType = (message?: string) => {
  return (value: null): ValidateCoreReturnType<null> => {
    if (value !== null) {
      return {
        validate: false,
        message: message ?? "",
        type: "null",
      };
    }
    return {
      validate: true,
      message: "",
      type: "null",
    };
  };
};
