/**
 * Number validation module for even numbers
 * Provides validation functionality for checking if a number is even
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a number is even
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<number>} - Validator for even numbers
 */
export const even = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value % 2 === 0,
  };
};
