import type { UnionToIntersection } from "$/logic";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

// Extract the validated value type by reading the validator's `type` field
// (and applying `ValidateType` to map type tags like "string" back to the
// runtime type). Reading the field directly lets validators that expose the
// literal union via the `type` field (such as `oneOf`) flow through
// intersection without being collapsed by `Types<T>`.
type ExtractValidatedType<V> = V extends (value: never) => { type: infer T }
  ? ValidateType<T>
  : never;

/**
 * Creates an intersection validator that passes only if all given validators pass
 * @param validators - Validator functions to compose as an intersection (logical AND)
 * @returns {Function} - Validator that checks if the value matches all validators
 */
export const intersection = <
  Vs extends ((value: never) => ValidateCoreReturnType<unknown>)[],
>(
  ...validators: [...Vs]
) => {
  return (
    value: UnionToIntersection<ExtractValidatedType<Vs[number]>>,
  ): ValidateCoreReturnType<
    UnionToIntersection<ExtractValidatedType<Vs[number]>>
  > => {
    for (const validator of validators) {
      const result = (
        validator as unknown as (
          v: UnionToIntersection<ExtractValidatedType<Vs[number]>>,
        ) => ValidateCoreReturnType<
          UnionToIntersection<ExtractValidatedType<Vs[number]>>
        >
      )(value);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type: value as unknown as Types<
            UnionToIntersection<ExtractValidatedType<Vs[number]>>
          >,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: value as unknown as Types<
        UnionToIntersection<ExtractValidatedType<Vs[number]>>
      >,
    };
  };
};
