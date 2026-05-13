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

// Walk the validator tuple and `&`-combine each element's extracted type.
// Reducing element-by-element preserves any union exposed by an individual
// validator (e.g. `union(A, B)`), unlike `UnionToIntersection` over
// `Vs[number]` which would distribute over the inner union and collapse
// `(A | B) & C` into `A & B & C`.
export type IntersectValidatedTypes<Vs> = Vs extends readonly [
  infer Head,
  ...infer Tail,
]
  ? ExtractValidatedType<Head> & IntersectValidatedTypes<Tail>
  : unknown;

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
    value: IntersectValidatedTypes<Vs>,
  ): ValidateCoreReturnType<IntersectValidatedTypes<Vs>> => {
    for (const validator of validators) {
      const result = (
        validator as unknown as (
          v: IntersectValidatedTypes<Vs>,
        ) => ValidateCoreReturnType<IntersectValidatedTypes<Vs>>
      )(value);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type: value as unknown as Types<IntersectValidatedTypes<Vs>>,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: value as unknown as Types<IntersectValidatedTypes<Vs>>,
    };
  };
};
