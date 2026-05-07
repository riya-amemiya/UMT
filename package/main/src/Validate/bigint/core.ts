/**
 * BigInt validation core module
 * Provides the base validation functionality for bigint values
 */

import type { ValidateReturnType } from "@/Validate/type";

/**
 * Return type produced by a `bigint` validator. Structurally compatible with
 * `ValidateCoreReturnType<bigint>`, but exposes the literal `"bigint"` tag
 * through the `type` field so `ValidateType<"bigint">` can map it back to the
 * `bigint` runtime type when consumed by `object()`, `union()`, and friends.
 */
export interface BigIntReturnType {
  validate: boolean;
  message: string;
  type: "bigint";
}

/**
 * Creates a bigint validator with optional validation rules
 * @template T - Array of validation rules for bigints
 * @param {T} [option] - Array of validation functions to apply
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a bigint and applies validation rules
 */
export const bigint = <T extends ValidateReturnType<bigint>[]>(
  option: T = [] as unknown as T,
  message?: string,
) => {
  return (value: bigint): BigIntReturnType => {
    if (typeof value !== "bigint") {
      return {
        validate: false,
        message: message ?? "",
        type: "bigint",
      };
    }
    for (const rule of option) {
      if (!rule.validate(value)) {
        return {
          validate: false,
          message: rule.message ?? "",
          type: "bigint",
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: "bigint",
    };
  };
};
