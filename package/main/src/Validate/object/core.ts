/**
 * Object validation core module
 * Provides validation functionality for objects with type-specific validation rules for each property
 */

import type { PickPartial } from "$/object";
import { isDictionaryObject } from "@/Validate/isDictionaryObject";
import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type {
  OptionalKeys,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

export type { StandardSchemaV1 } from "@/Validate/standardSchema";

/**
 * Shape map describing per-property validators consumed by `object()`
 */
export interface ObjectShape {
  // biome-ignore lint/suspicious/noExplicitAny: Required for flexible object property validation
  [key: string]: (value: any) => ValidateCoreReturnType<unknown>;
}

/**
 * Resolved property map produced by applying `ValidateType` to every shape
 * entry. Each property maps a `ValidateType` tag (such as `"string"`,
 * `"number"`) back to the runtime type expected at that key.
 */
export type ObjectShapeProperties<T extends ObjectShape> = {
  [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
};

/**
 * Inferred object type produced by `object()`. Splits the property map into
 * required and optional halves via `PickPartial` + `OptionalKeys` so optional
 * properties surface with `?:` in the resulting type.
 */
export type InferObject<T extends ObjectShape> = {
  [key in keyof PickPartial<
    ObjectShapeProperties<T>,
    OptionalKeys<ObjectShapeProperties<T>>
  >]: PickPartial<
    ObjectShapeProperties<T>,
    OptionalKeys<ObjectShapeProperties<T>>
  >[key];
};

/**
 * Object validator augmented with the original `shape` map. The shape map is
 * exposed so derived helpers such as `pick()`, `omit()`, `partial()`, and
 * `required()` can compose new object validators from existing ones.
 */
export type ObjectValidator<T extends ObjectShape> = ((
  value: InferObject<T>,
) => ValidateCoreReturnType<InferObject<T>>) & {
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
): ObjectValidator<T> & StandardSchemaV1<InferObject<T>, InferObject<T>> => {
  type Inferred = InferObject<T>;

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
  return attachStandard<Inferred, Inferred, typeof validator>(validator);
};
