/**
 * Unknown validation core module
 * Provides a validator that accepts any value but exposes it as `unknown`
 * to keep callers honest about narrowing before use.
 */

import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";

/**
 * Return type produced by an `unknown` validator. Exposes the literal
 * `"unknown"` tag through the `type` field so `ValidateType<"unknown">` can
 * map it back to the `unknown` runtime type when consumed by downstream
 * helpers.
 */
export interface UnknownReturnType {
  validate: boolean;
  message: string;
  type: "unknown";
}

const unknownValidator = (_value: unknown): UnknownReturnType => ({
  validate: true,
  message: "",
  type: "unknown",
});

const standardUnknownValidator = attachStandard<
  unknown,
  unknown,
  typeof unknownValidator
>(unknownValidator);

/**
 * Creates a validator that accepts any value but typed as unknown
 * @returns {Function} - Validator that always succeeds
 */
export const unknown = (): ((value: unknown) => UnknownReturnType) &
  StandardSchemaV1<unknown, unknown> => standardUnknownValidator;
