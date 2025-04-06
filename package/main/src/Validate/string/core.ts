/**
 * String validation core module
 * Provides the base validation functionality for string values
 */

import { core } from "@/Validate/core";
import type { ValidateReturnType } from "@/Validate/type";

/**
 * Creates a string validator with optional validation rules
 * @template T - Array of validation rules for strings
 * @param {T} [option] - Array of validation functions to apply
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a string and applies validation rules
 */
export const string =
  <T extends ValidateReturnType<string>[]>(option?: T, message?: string) =>
  (value: string) =>
    core<string>("string")(value, option, message);
