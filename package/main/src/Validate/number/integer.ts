/**
Number validation module for integer check
Provides validation functionality for checking if a number is an integer
*/

import type { ValidateReturnType } from "@/Validate/type";

/**
Creates a validator for checking if a number is an integer
@param {string} [message] - Custom error message for validation failure
@returns {ValidateReturnType<number>} - Validator for integer numbers
*/
export const integer = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => Number.isInteger(value),
  };
};
