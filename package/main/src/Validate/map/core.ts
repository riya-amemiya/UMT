/**
 * Map validation core module
 * Provides validation for `Map` instances. The validator can optionally
 * delegate to per-entry validators for keys and values, mirroring how
 * `arrayOf()` validates each element of an array.
 */

import type { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

type ExtractValidatedType<V> = V extends (value: never) => { type: infer T }
  ? ValidateType<T>
  : never;

/**
 * Creates a Map validator. The validator can optionally accept per-entry key
 * and value validators that are applied to every entry of the map. When the
 * value-side validator is provided, the entry validators short-circuit on the
 * first failure and surface the failing message, mirroring `arrayOf()`.
 * @template KV - Validator for the map key
 * @template VV - Validator for the map value
 * @param {KV} [keyValidator] - Validator applied to every key
 * @param {VV} [valueValidator] - Validator applied to every value
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function for Map instances
 */
export const map = <
  // biome-ignore lint/suspicious/noExplicitAny: validator inputs vary
  KV extends (value: any) => ValidateCoreReturnType<unknown> = (
    value: unknown,
  ) => ValidateCoreReturnType<unknown>,
  // biome-ignore lint/suspicious/noExplicitAny: validator inputs vary
  VV extends (value: any) => ValidateCoreReturnType<unknown> = (
    value: unknown,
  ) => ValidateCoreReturnType<unknown>,
  K = ExtractValidatedType<KV>,
  V = ExtractValidatedType<VV>,
>(
  keyValidator?: KV,
  valueValidator?: VV,
  message?: string,
) => {
  return (value: Map<K, V>): ValidateCoreReturnType<Map<K, V>> => {
    if (!(value instanceof Map)) {
      return {
        validate: false,
        message: message ?? "",
        type: value,
      };
    }
    for (const [entryKey, entryValue] of value) {
      if (keyValidator) {
        const keyResult = (
          keyValidator as (v: unknown) => { validate: boolean; message: string }
        )(entryKey);
        if (!keyResult.validate) {
          return {
            validate: false,
            message: keyResult.message,
            type: value,
          };
        }
      }
      if (valueValidator) {
        const valueResult = (
          valueValidator as (v: unknown) => {
            validate: boolean;
            message: string;
          }
        )(entryValue);
        if (!valueResult.validate) {
          return {
            validate: false,
            message: valueResult.message,
            type: value,
          };
        }
      }
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  };
};
