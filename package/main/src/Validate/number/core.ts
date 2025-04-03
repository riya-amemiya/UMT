/**
 * Number validation core module
 * Provides the base validation functionality for number values
 */

import { core } from "@/Validate/core";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a number validator with optional validation rules
 * @template T - Array of validation rules for numbers
 * @param {T} [option] - Array of validation functions to apply
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a number and applies validation rules
 */
export const number = <T extends ValidateReturnType<number>[]>(
  option?: T,
  message?: string,
) => {
  return (value: number) => core<number>("number")(value, option, message);
};
