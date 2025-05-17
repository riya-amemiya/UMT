/**
 * Object validation core module
 * Provides validation functionality for objects with type-specific validation rules for each property
 */

import { isDictionaryObject } from "@/Validate/isDictionaryObject";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

/**
 * Creates an object validator with property-specific validation rules
 * @template T - Object type containing validation functions for each property
 * @param {T} [option] - Object containing validation functions for each property
 * @param {string} [message] - Custom error message for object type validation
 * @returns {Function} - Validator function that checks if the value is an object and validates its properties
 */
export const object = <
  T extends {
    // biome-ignore lint/suspicious/noExplicitAny: Required for flexible object property validation
    [key: string]: (value: any) => ValidateCoreReturnType<any>;
  },
>(
  option: T = {} as T,
  message?: string,
) => {
  return (
    value: Types<{
      [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
    }>,
  ): ValidateCoreReturnType<{
    [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
  }> => {
    if (!isDictionaryObject(value)) {
      return {
        validate: false,
        message: message || "",
        type: value,
      };
    }
    for (const validate in option) {
      if (!option[validate](value[validate]).validate) {
        return {
          validate: false,
          message: option[validate](value).message,
          type: value,
        };
      }
    }

    return {
      validate: true,
      message: "",
      type: value,
    };
  };
};
