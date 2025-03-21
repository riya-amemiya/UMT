/**
 * String validation module for email addresses
 * Provides validation functionality for checking if a string is a valid email address
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string is a valid email address
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for email addresses
 */
export const email = (message?: string): ValidateReturnType<string> => {
  // Regular expression for email address validation
  const emailRegex =
    /^[\w+-]+(?:\.[\w+-]+)*@[\da-z]+(?:[.-][\da-z]+)*\.[a-z]{2,}$/iu;
  return {
    type: "string",
    message,
    validate: (value) => emailRegex.test(value),
  };
};
