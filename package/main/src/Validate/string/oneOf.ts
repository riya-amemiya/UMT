/**
 * String validation module for literal union (allowed values) check
 * Provides validation functionality for checking if a string is one of the
 * allowed literal values, useful for validating string literal union types
 * such as `'standard' | 'squat' | 'decanter' | 'round' | 'tall' | 'flask'`.
 *
 * Unlike string rules used inside `string([...])`, this is a top-level
 * validator that preserves the literal union in its generic parameter, so
 * the resulting validator carries the union type through the type system.
 */

import type { Types, ValidateCoreReturnType } from "@/Validate/type";

/**
 * Creates a top-level validator that checks if a string value is one of the
 * given allowed literal values. The literal union is captured via `const T`
 * and exposed through the `ValidateCoreReturnType<T[number]>` return type.
 * @template T - The tuple of allowed string literals
 * @param {T} values - The tuple of allowed string values
 * @param {string} [message] - Custom error message for validation failure
 * @returns A validator function from `string` to `ValidateCoreReturnType<T[number]>`
 */
export const oneOf = <const T extends readonly string[]>(
  values: T,
  message?: string,
) => {
  const allowed = new Set<string>(values);
  return (value: string): ValidateCoreReturnType<T[number]> => {
    const isValid = allowed.has(value);
    return {
      validate: isValid,
      message: isValid ? "" : (message ?? ""),
      type: "string" as Types<T[number]>,
    };
  };
};
