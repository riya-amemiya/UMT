/**
 * String validation module for checking if a string represents a number
 * Provides validation functionality for checking if a string can be converted to a number
 */

import { isNumber } from "@/Validate/isNumber";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string represents a valid number
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for number strings
 */
export const numberString = (message?: string): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value) => isNumber(value),
  };
};
