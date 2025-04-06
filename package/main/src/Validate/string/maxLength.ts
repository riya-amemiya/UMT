/**
 * String validation module for maximum length check
 * Provides validation functionality for checking if a string's length is less than or equal to a maximum value
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string's length is less than or equal to a maximum value
 * @param {number} maxLength - Maximum allowed length of the string
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for maximum string length
 */
export const maxLength = (
  maxLength: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length <= maxLength,
});
