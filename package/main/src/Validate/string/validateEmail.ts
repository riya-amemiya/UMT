/**
 * String validation module for email addresses
 * Provides validation functionality for checking if a string is a valid email address
 */

import { parseEmail, type ParseEmailOptions } from "@/Validate/parseEmail";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string is a valid email address
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for email addresses
 */
export const validateEmail = (
  message?: string,
  options?: ParseEmailOptions,
): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value) => {
      return parseEmail({
        email: value,
        options: options ?? { level: "basic" },
      }).valid;
    },
  };
};
