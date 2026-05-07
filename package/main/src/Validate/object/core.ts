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
 * Shape map describing per-property validators consumed by `object()`
 */
export interface ObjectShape {
  // biome-ignore lint/suspicious/noExplicitAny: Required for flexible object property validation
  [key: string]: (value: any) => ValidateCoreReturnType<unknown>;
}

/**
 * Object validator augmented with the original `shape` map. The shape map is
 * exposed so derived helpers such as `pick()`, `omit()`, `partial()`, and
 * `required()` can compose new object validators from existing ones.
 */
export type ObjectValidator<
  T extends ObjectShape,
  U = {
    [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
  },
> = ((
  value: {
    [key in keyof PickPartial<U, OptionalKeys<U>>]: PickPartial<
      U,
      OptionalKeys<U>
    >[key];
  },
) => ValidateCoreReturnType<{
  [key in keyof PickPartial<U, OptionalKeys<U>>]: PickPartial<
    U,
    OptionalKeys<U>
  >[key];
}>) & {
  shape: T;
};

/**
 * Creates an object validator with property-specific validation rules
 * @template T - Object type containing validation functions for each property
 * @param {T} [option] - Object containing validation functions for each property
 * @param {string} [message] - Custom error message for object type validation
 * @returns {ObjectValidator<T>} - Validator function with `.shape` attached so it can compose with `pick()`, `omit()`, `partial()`, and `required()`
 */
export const object = <T extends ObjectShape>(
  option: T = {} as T,
  message?: string,
): ObjectValidator<T> => {
  type U = {
    [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
  };
  type Inferred = {
    [key in keyof PickPartial<U, OptionalKeys<U>>]: PickPartial<
      U,
      OptionalKeys<U>
    >[key];
  };

  const validator = ((value: Inferred): ValidateCoreReturnType<Inferred> => {
    if (!isDictionaryObject(value)) {
      return {
        validate: false,
        message: message ?? "",
        type: value,
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
  }) as ObjectValidator<T>;

  validator.shape = option;
  return validator;
};
