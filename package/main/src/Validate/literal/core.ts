/**
Literal validation core module
Provides validation for a single primitive literal value, compared with
strict equality (`===`)
*/

import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type { LiteralBrand } from "@/Validate/type";

type LiteralValue = string | number | boolean | bigint | null | undefined;

type LiteralTag<T extends LiteralValue> = T extends string
  ? LiteralBrand<T>
  : T;

interface LiteralResult<T extends LiteralValue> {
  validate: boolean;
  message: string;
  type: LiteralTag<T>;
}

/**
Creates a validator that accepts exactly one primitive value. The value is
compared with strict equality (`===`), so `literal(NaN)` never matches and
`literal(0)` matches `-0`. String literals are carried through the `type`
field as a `LiteralBrand` so they are not confused with reserved type tags
such as `"string"` when composed inside `object()`, `union()`, or `arrayOf()`.
@template T - The literal value accepted by the validator
@param {T} expected - The only value accepted by the validator
@param {string} [message] - Custom error message for validation failure
@returns {Function} - Validator function that checks if the value is strictly equal to `expected`
*/
export const literal = <const T extends LiteralValue>(
  expected: T,
  message = "",
): ((value: T) => LiteralResult<T>) & StandardSchemaV1<T, T> => {
  const literalValidator = (value: T): LiteralResult<T> => {
    const isValid = value === expected;
    return {
      validate: isValid,
      message: isValid ? "" : message,
      type: value as LiteralTag<T>,
    };
  };
  return attachStandard<T, T, typeof literalValidator>(literalValidator);
};
