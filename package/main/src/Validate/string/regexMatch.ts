/**
 * String validation module for regular expression matching
 * Provides validation functionality for checking if a string matches a regular expression pattern
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string matches a regular expression pattern
 * @param {RegExp} pattern - Regular expression pattern to match against
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for regular expression matching
 */
export const regexMatch = (
  pattern: RegExp,
  message?: string,
): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value: string) => pattern.test(value),
  };
};
