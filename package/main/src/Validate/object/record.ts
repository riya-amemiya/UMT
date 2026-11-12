import { isDictionaryObject } from "@/Validate/isDictionaryObject";
import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

type RecordEntryValidator = (value: never) => {
  type: unknown;
  message: string;
  validate: boolean;
};

type RecordExtractValidatedType<V> = V extends (value: never) => {
  type: infer T;
}
  ? ValidateType<T>
  : never;

type RecordInfer<
  KV extends RecordEntryValidator,
  VV extends RecordEntryValidator,
> = Record<
  RecordExtractValidatedType<KV> & PropertyKey,
  RecordExtractValidatedType<VV>
>;

/**
Creates a record validator for dictionary objects with dynamic keys. Every own
enumerable key (as returned by `Object.keys`) is validated with
`keyValidator`, then its value with `valueValidator`; iteration
short-circuits at the first failure and surfaces the failing message.
@template KV - Validator applied to every key
@template VV - Validator applied to every value
@param {KV} keyValidator - Validator applied to every key (e.g. `string()` or `oneOf()`)
@param {VV} valueValidator - Validator applied to every value
@param {string} [message] - Custom error message when the value is not a dictionary object
@returns {Function} - Validator function for dictionary objects whose entries satisfy both validators
*/
export const record = <
  KV extends RecordEntryValidator,
  VV extends RecordEntryValidator,
>(
  keyValidator: KV,
  valueValidator: VV,
  message = "",
): ((
  value: RecordInfer<KV, VV>,
) => ValidateCoreReturnType<RecordInfer<KV, VV>>) &
  StandardSchemaV1<RecordInfer<KV, VV>, RecordInfer<KV, VV>> => {
  type Inferred = RecordInfer<KV, VV>;
  const validateKey = keyValidator as unknown as (v: string) => {
    validate: boolean;
    message: string;
  };
  const validateValue = valueValidator as unknown as (v: unknown) => {
    validate: boolean;
    message: string;
  };
  const recordValidator = (
    value: Inferred,
  ): ValidateCoreReturnType<Inferred> => {
    const type = value as unknown as Types<Inferred>;
    const entries: unknown = value;
    if (!isDictionaryObject<Record<string, unknown>>(entries)) {
      return {
        validate: false,
        message,
        type,
      };
    }
    for (const key of Object.keys(entries)) {
      const keyResult = validateKey(key);
      if (!keyResult.validate) {
        return {
          validate: false,
          message: keyResult.message,
          type,
        };
      }
      const valueResult = validateValue(entries[key]);
      if (!valueResult.validate) {
        return {
          validate: false,
          message: valueResult.message,
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
  return attachStandard<Inferred, Inferred, typeof recordValidator>(
    recordValidator,
  );
};
