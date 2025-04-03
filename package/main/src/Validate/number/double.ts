/**
 * Number validation module for double (floating point) numbers
 * Provides validation functionality for checking if a number is a floating point value
 */

import { isDouble } from "@/Validate/isDouble";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a number is a floating point value
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<number>} - Validator for double numbers
 */
export const double = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isDouble(value),
  };
};
