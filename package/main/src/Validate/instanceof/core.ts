/**
 * instanceof validation core module
 * Provides a validator that checks whether a value is an instance of a
 * specific constructor, including subclasses. The validator is exported as
 * `instanceof_` because `instanceof` is a reserved keyword in JavaScript.
 */

import type { Types, ValidateCoreReturnType } from "@/Validate/type";

// biome-ignore lint/suspicious/noExplicitAny: a constructor signature must accept any tuple
export type Constructor<T> = new (...arguments_: any[]) => T;

/**
 * Creates a validator that checks whether a value is an instance of the given
 * constructor. Subclasses of the constructor satisfy the validator, matching
 * native `instanceof` semantics.
 * @template T - The instance type produced by the constructor
 * @param {Constructor<T>} classConstructor - Constructor whose instances are accepted
 * @param {string} [message] - Custom error message for validation failure
 * @returns {Function} - Validator function for instances of the constructor
 */
export const instanceof_ = <T>(
  classConstructor: Constructor<T>,
  message?: string,
) => {
  return (value: T): ValidateCoreReturnType<T> => {
    if (!(value instanceof classConstructor)) {
      return {
        validate: false,
        message: message ?? "",
        type: value as unknown as Types<T>,
      };
    }
    return {
      validate: true,
      message: "",
      type: value as unknown as Types<T>,
    };
  };
};
