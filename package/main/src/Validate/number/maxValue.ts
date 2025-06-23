/**
 * Number validation module for maximum value check
 * Provides validation functionality for checking if a number is less than or equal to a maximum value
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a number is less than or equal to a maximum value
 * @param {number} maxValue - Maximum allowed value
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<number>} - Validator for maximum value check
 */
export const maxValue = (
  maxValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value <= maxValue,
  };
};
