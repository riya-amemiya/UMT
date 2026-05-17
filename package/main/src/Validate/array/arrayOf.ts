import { isArray } from "@/Validate/isArray";
import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

// Extract the validated value type by reading the validator's `type` field
// (and applying `ValidateType` to map type tags like "string" back to the
// runtime type). Reading the field directly lets validators that expose the
// literal union via the `type` field (such as `oneOf`) flow through arrayOf
// without being collapsed to `string`.
export type ArrayOfExtractValidatedType<V> = V extends (value: never) => {
  type: infer T;
}
  ? ValidateType<T>
  : never;

/**
 * Creates an array validator that validates every element with a single validator
 * @template V - The validator function applied to each element (its return `type` field carries the element type)
 * @param {V} validator - Validator applied to each element (e.g. an object validator)
 * @param {string} [message] - Custom error message for array type validation
 * @returns {Function} - Validator function for arrays whose elements satisfy the given validator
 */
export const arrayOf = <
  V extends (value: never) => {
    type: unknown;
    message: string;
    validate: boolean;
  },
>(
  validator: V,
  message?: string,
): ((
  values: ArrayOfExtractValidatedType<V>[],
) => ValidateCoreReturnType<ArrayOfExtractValidatedType<V>[]>) &
  StandardSchemaV1<
    ArrayOfExtractValidatedType<V>[],
    ArrayOfExtractValidatedType<V>[]
  > => {
  type Element = ArrayOfExtractValidatedType<V>;
  const arrayValidator = (
    values: Element[],
  ): ValidateCoreReturnType<Element[]> => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message ?? "",
        type: values,
      };
    }
    for (const value of values) {
      const result = (
        validator as unknown as (v: Element) => {
          validate: boolean;
          message: string;
        }
      )(value);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type: values,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: values,
    };
  };
  return attachStandard<Element[], Element[], typeof arrayValidator>(
    arrayValidator,
  );
};
