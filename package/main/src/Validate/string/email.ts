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
    /^[a-zA-Z0-9]([a-zA-Z0-9._+-]*[a-zA-Z0-9])?@[a-zA-Z0-9]([a-zA-Z0-9.-]*[a-zA-Z0-9])?\.[a-zA-Z]{2,}$/;
  return {
    type: "string",
    message,
    validate: (value) => {
      // RFC 5322 length limitations
      if (value.length > 998) {
        return false;
      }

      // Check for consecutive dots
      if (value.includes("..")) {
        return false;
      }
      // Check for leading/trailing dots in local part
      const [localPart, domainPart] = value.split("@");
      if (!(localPart && domainPart)) {
        return false;
      }

      // RFC 5321 length limits: local part max 64 chars, domain part max 253 chars
      if (localPart.length > 64 || domainPart.length > 253) {
        return false;
      }

      if (localPart.startsWith(".") || localPart.endsWith(".")) {
        return false;
      }
      if (domainPart.startsWith(".") || domainPart.endsWith(".")) {
        return false;
      }
      // Use regex for final validation
      return emailRegex.test(value);
    },
  };
};
