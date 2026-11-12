import { isArray } from "@/Validate/isArray";
import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

type TupleElementValidator = (value: never) => {
  type: unknown;
  message: string;
  validate: boolean;
};

type TupleExtractValidatedType<V> = V extends (value: never) => {
  type: infer T;
}
  ? ValidateType<T>
  : never;

type TupleInfer<Vs extends readonly TupleElementValidator[]> = {
  -readonly [K in keyof Vs]: TupleExtractValidatedType<Vs[K]>;
};

/**
Creates a tuple validator for fixed-length arrays whose elements are validated
position by position with their own validator
@template Vs - The tuple of validators, one per position (each return `type` field carries the element type)
@param {Vs} validators - Validators applied to the element at the same index
@param {string} [message] - Custom error message when the value is not an array or its length differs from the number of validators
@returns {Function} - Validator function for arrays of exactly `validators.length` elements that satisfy the validator at their index
*/
export const tuple = <const Vs extends readonly TupleElementValidator[]>(
  validators: Vs,
  message = "",
): ((values: TupleInfer<Vs>) => ValidateCoreReturnType<TupleInfer<Vs>>) &
  StandardSchemaV1<TupleInfer<Vs>, TupleInfer<Vs>> => {
  type Inferred = TupleInfer<Vs>;
  const length = validators.length;
  const elementValidators = validators as unknown as readonly ((v: unknown) => {
    validate: boolean;
    message: string;
  })[];
  const tupleValidator = (
    values: Inferred,
  ): ValidateCoreReturnType<Inferred> => {
    const type = values as unknown as Types<Inferred>;
    if (!isArray<unknown>(values) || values.length !== length) {
      return {
        validate: false,
        message,
        type,
      };
    }
    for (let index = 0; index < length; index++) {
      const result = elementValidators[index](values[index]);
      if (!result.validate) {
        return {
          validate: false,
          message: result.message,
          type,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type,
    };
  };
  return attachStandard<Inferred, Inferred, typeof tupleValidator>(
    tupleValidator,
  );
};
