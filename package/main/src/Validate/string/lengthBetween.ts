/**
String validation module for inclusive length range check
Provides validation functionality for checking if a string's length lies between a minimum and a maximum value
*/

import type { ValidateReturnType } from "@/Validate/type";

/**
Creates a validator for checking if a string's length is greater than or equal to a minimum value and less than or equal to a maximum value
@param {number} min - Minimum allowed length of the string (inclusive)
@param {number} max - Maximum allowed length of the string (inclusive)
@param {string} [message] - Custom error message for validation failure
@returns {ValidateReturnType<string>} - Validator for inclusive string length range
*/
export const lengthBetween = (
  min: number,
  max: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length >= min && value.length <= max,
});
