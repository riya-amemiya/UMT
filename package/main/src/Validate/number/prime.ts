/**
 * Number validation module for prime numbers
 * Provides validation functionality for checking if a number is prime
 */

import { isPrimeNumber } from "@/Validate/isPrimeNumber";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a number is prime
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<number>} - Validator for prime numbers
 */
export const prime = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isPrimeNumber(value),
  };
};
