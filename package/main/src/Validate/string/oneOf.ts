/**
 * String validation module for literal union (allowed values) check
 * Provides validation functionality for checking if a string is one of the
 * allowed literal values, useful for validating string literal union types
 * such as `'standard' | 'squat' | 'decanter' | 'round' | 'tall' | 'flask'`.
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a validator for checking if a string is included in the given
 * allowed values. Equivalent to checking membership in a string literal
 * union type.
 * @template T - The allowed string literal type
 * @param {readonly T[]} values - The list of allowed string values
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for allowed string values
 */
export const oneOf = <T extends string>(
  values: readonly T[],
  message?: string,
): ValidateReturnType<string> => {
  const allowed = new Set<string>(values);
  return {
    type: "string",
    message,
    validate: (value) => allowed.has(value),
  };
};
