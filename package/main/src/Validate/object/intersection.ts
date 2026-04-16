import type { UnionToIntersection } from "$/logic";
import type { Types, ValidateCoreReturnType } from "@/Validate/type";

type ExtractValidatedType<V> = V extends (
  value: never,
) => ValidateCoreReturnType<infer T>
  ? T
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
