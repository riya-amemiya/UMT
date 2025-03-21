/**
 * Number validation module for minimum value check
 * Provides validation functionality for checking if a number is greater than or equal to a minimum value
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a number is greater than or equal to a minimum value
 * @param {number} minValue - Minimum allowed value
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<number>} - Validator for minimum value check
 */
export const minValue = (
  minValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value >= minValue,
  };
};
