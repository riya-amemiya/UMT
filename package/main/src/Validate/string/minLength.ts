/**
 * String validation module for minimum length check
 * Provides validation functionality for checking if a string's length is greater than or equal to a minimum value
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string's length is greater than or equal to a minimum value
 * @param {number} minLength - Minimum allowed length of the string
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for minimum string length
 */
export const minLength = (
  minLength: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length >= minLength,
});
