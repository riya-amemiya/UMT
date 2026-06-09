/**
 * String validation core module
 * Provides the base validation functionality for string values
 */

import { core } from "@/Validate/core";
import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type {
  ValidateCoreReturnType,
  ValidateReturnType,
} from "@/Validate/type";

/**
 * Creates a string validator with optional validation rules
 * @template T - Array of validation rules for strings
 * @param {T} [option] - Array of validation functions to apply
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a string and applies validation rules
 */
export const string = <T extends ValidateReturnType<string>[]>(
  option?: T,
  message?: string,
): ((value: string) => ValidateCoreReturnType<string>) &
  StandardSchemaV1<string, string> => {
  // Build the core validator and resolve the rules array once so each call
  // avoids re-creating the curried closure and a fresh default array.
  const rules = option ?? [];
  const run = core<string>("string");
  const validator = (value: string) => run(value, rules, message);
  return attachStandard<string, string, typeof validator>(validator);
};
