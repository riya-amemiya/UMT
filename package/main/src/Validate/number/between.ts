/**
Number validation module for inclusive range check
Provides validation functionality for checking if a number lies between a minimum and a maximum value
*/

import type { ValidateReturnType } from "@/Validate/type";

/**
Creates a validator for checking if a number is greater than or equal to a minimum value and less than or equal to a maximum value
@param {number} min - Minimum allowed value (inclusive)
@param {number} max - Maximum allowed value (inclusive)
@param {string} [message] - Custom error message for validation failure
@returns {ValidateReturnType<number>} - Validator for inclusive range check
*/
export const between = (
  min: number,
  max: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value >= min && value <= max,
  };
};
