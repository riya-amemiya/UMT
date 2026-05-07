/**
 * Date validation core module
 * Provides the base validation functionality for Date instances
 */

import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Creates a Date validator. The validator checks both that the value is an
 * instance of Date and that it represents a valid moment in time (i.e. its
 * underlying timestamp is not NaN), so values created from invalid inputs
 * such as `new Date("not-a-date")` are rejected.
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a Date instance
 */
export const date = (message?: string) => {
  return (value: Date): ValidateCoreReturnType<Date> => {
    if (!(value instanceof Date) || Number.isNaN(value.getTime())) {
      return {
        validate: false,
        message: message ?? "",
        type: value,
      };
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  };
};
