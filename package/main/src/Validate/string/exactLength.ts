/**
 * String validation module for exact length check
 * Provides validation functionality for checking if a string has an exact length
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string has an exact length
 * @param {number} length - The exact length that the string should have
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for string length
 */
export const exactLength = (
  length: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length === length,
});
