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
    /^(?=.{1,998}$)(?!.*\.\.)(?<local>(?<localName>[a-zA-Z0-9](?:[a-zA-Z0-9._-]{0,62}[a-zA-Z0-9])?)(?:\+(?<alias>[a-zA-Z0-9._+-]+))?)@(?<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*\.[a-zA-Z]{2,})$/;
  return {
    type: "string",
    message,
    validate: (value) => {
      return emailRegex.test(value);
    },
  };
};
