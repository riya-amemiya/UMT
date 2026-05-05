import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

// Extract the validated value type by reading the validator's `type` field
// (and applying `ValidateType` to map type tags like "string" back to the
// runtime type). Reading the field directly lets validators that expose the
// literal union via the `type` field (such as `oneOf`) flow through union
// without being collapsed by `Types<T>`.
type ExtractValidatedType<V> = V extends (value: never) => { type: infer T }
  ? ValidateType<T>
  : never;

/**
 * Creates a union validator that passes if any of the given validators pass
 * @param validators - Validator functions to compose as a union (logical OR)
 * @returns {Function} - Validator that checks if the value matches any of the validators
 */
export const union = <
  Vs extends ((value: never) => ValidateCoreReturnType<unknown>)[],
>(
  ...validators: [...Vs]
) => {
  return (
    value: ExtractValidatedType<Vs[number]>,
  ): ValidateCoreReturnType<ExtractValidatedType<Vs[number]>> => {
    let lastMessage = "";
    for (const validator of validators) {
      const result = (
        validator as unknown as (
          v: ExtractValidatedType<Vs[number]>,
        ) => ValidateCoreReturnType<ExtractValidatedType<Vs[number]>>
      )(value);
      if (result.validate) {
        return {
          validate: true,
          message: "",
          type: value as unknown as Types<ExtractValidatedType<Vs[number]>>,
        };
      }
      lastMessage = result.message;
    }
    return {
      validate: false,
      message: lastMessage,
      type: value as unknown as Types<ExtractValidatedType<Vs[number]>>,
    };
  };
};
