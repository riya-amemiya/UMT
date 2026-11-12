/**
String validation module for URL strings
Provides validation functionality for checking if a string is an absolute URL
*/

import { isAbsoluteUrl } from "@/URL/isAbsoluteUrl";
import type { ValidateReturnType } from "@/Validate/type";

/**
Creates a validator for checking if a string is an absolute URL (RFC 3986 scheme followed by a colon)
@param {string} [message] - Custom error message for validation failure
@returns {ValidateReturnType<string>} - Validator for URL strings
*/
export const url = (message?: string): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => isAbsoluteUrl(value),
});
