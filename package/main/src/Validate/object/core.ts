/**
 * Object validation core module
 * Provides validation functionality for objects with type-specific validation rules for each property
 */

import type { PickPartial } from "$/object";
import { isDictionaryObject } from "@/Validate/isDictionaryObject";
import type {
  OptionalKeys,
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
  U = {
    [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
  },
>(
  option: T = {} as T,
  message?: string,
) => {
  return (
    value: {
      [key in keyof PickPartial<U, OptionalKeys<U>>]: PickPartial<
        U,
        OptionalKeys<U>
      >[key];
    },
  ): ValidateCoreReturnType<{
    [key in keyof PickPartial<U, OptionalKeys<U>>]: PickPartial<
      U,
      OptionalKeys<U>
    >[key];
  }> => {
    if (!isDictionaryObject(value)) {
      return {
        validate: false,
        message: message || "",
        // biome-ignore lint/suspicious/noExplicitAny: Type assertion needed for return type compatibility
        type: value as any,
      };
    }
    for (const validate in option) {
      // biome-ignore lint/suspicious/noExplicitAny: Index access requires any cast
      if (!option[validate]((value as any)[validate]).validate) {
        return {
          validate: false,
          // biome-ignore lint/suspicious/noExplicitAny: Index access requires any cast
          message: option[validate](value as any).message,
          // biome-ignore lint/suspicious/noExplicitAny: Type assertion needed for return type compatibility
          type: value as any,
        };
      }
    }

    return {
      validate: true,
      message: "",
      // biome-ignore lint/suspicious/noExplicitAny: Type assertion needed for return type compatibility
      type: value as any,
    };
  };
};
