/**
 * Any validation core module
 * Provides a validator that accepts any value, useful when a position in a
 * schema needs to remain wide open while still participating in `object()`,
 * `union()`, `intersection()`, and other compositional helpers.
 */

import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";

/**
 * Return type produced by an `any` validator. Exposes the literal `"any"`
 * tag through the `type` field so `ValidateType<"any">` can map it back to
 * the `any` runtime type when consumed by downstream helpers.
 */
export interface AnyReturnType {
  validate: boolean;
  message: string;
  type: "any";
}

// biome-ignore lint/suspicious/noExplicitAny: signature mirrors the return type
const anyValidator = (_value: any): AnyReturnType => ({
  validate: true,
  message: "",
  type: "any",
});

const standardAnyValidator = attachStandard<
  // biome-ignore lint/suspicious/noExplicitAny: any() carries any through Standard Schema
  any,
  // biome-ignore lint/suspicious/noExplicitAny: any() carries any through Standard Schema
  any,
  typeof anyValidator
>(anyValidator);

/**
 * Creates a validator that accepts any value
 * @returns {Function} - Validator that always succeeds
 */
export const any = (): ((
  // biome-ignore lint/suspicious/noExplicitAny: any() must accept and infer any value
  value: any,
) => AnyReturnType) &
  // biome-ignore lint/suspicious/noExplicitAny: Standard Schema infers any in/out for any()
  StandardSchemaV1<any, any> => standardAnyValidator;
