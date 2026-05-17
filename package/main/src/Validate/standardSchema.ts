/**
 * Standard Schema V1 integration
 *
 * Defines the public Standard Schema V1 surface (https://standardschema.dev)
 * and exposes the `attachStandard()` helper used by every UMT validator
 * factory to advertise itself as a Standard Schema V1 implementation. The
 * `~standard` property added by `attachStandard()` lets external tools
 * (Zod-, Valibot-, ArkType-style ecosystems) consume UMT validators without
 * adapters.
 *
 * Biome's `noNamespace` lint rule forbids TypeScript namespaces, so the spec
 * is mirrored with flat interface/type names that remain structurally
 * compatible with the official `StandardSchemaV1` interface published by
 * `@standard-schema/spec`.
 */

/** The Standard Schema V1 interface. */
export interface StandardSchemaV1<Input = unknown, Output = Input> {
  readonly "~standard": StandardSchemaV1Properties<Input, Output>;
}

/** The Standard Schema V1 properties interface. */
export interface StandardSchemaV1Properties<Input = unknown, Output = Input> {
  /** The version number of the standard. */
  readonly version: 1;
  /** The vendor name of the schema library. */
  readonly vendor: string;
  /** Validates unknown input values. */
  readonly validate: (
    value: unknown,
  ) => StandardSchemaV1Result<Output> | Promise<StandardSchemaV1Result<Output>>;
  /** Inferred types associated with the schema. */
  readonly types?: StandardSchemaV1Types<Input, Output> | undefined;
}

/** The result type produced by `Props.validate`. */
export type StandardSchemaV1Result<Output> =
  | StandardSchemaV1SuccessResult<Output>
  | StandardSchemaV1FailureResult;

/** The result interface when validation succeeds. */
export interface StandardSchemaV1SuccessResult<Output> {
  readonly value: Output;
  readonly issues?: undefined;
}

/** The result interface when validation fails. */
export interface StandardSchemaV1FailureResult {
  readonly issues: readonly StandardSchemaV1Issue[];
}

/** The issue interface returned on failure. */
export interface StandardSchemaV1Issue {
  readonly message: string;
  readonly path?:
    | readonly (PropertyKey | StandardSchemaV1PathSegment)[]
    | undefined;
}

/** The path segment interface for nested issues. */
export interface StandardSchemaV1PathSegment {
  readonly key: PropertyKey;
}

/** The Standard Schema V1 types interface. */
export interface StandardSchemaV1Types<Input = unknown, Output = Input> {
  /** The input type of the schema. */
  readonly input: Input;
  /** The output type of the schema. */
  readonly output: Output;
}

/** Infers the input type of a Standard Schema V1 implementation. */
export type StandardSchemaV1InferInput<
  // biome-ignore lint/suspicious/noExplicitAny: schema may be any spec-compliant value
  Schema extends StandardSchemaV1<any, any>,
> = NonNullable<Schema["~standard"]["types"]>["input"];

/** Infers the output type of a Standard Schema V1 implementation. */
export type StandardSchemaV1InferOutput<
  // biome-ignore lint/suspicious/noExplicitAny: schema may be any spec-compliant value
  Schema extends StandardSchemaV1<any, any>,
> = NonNullable<Schema["~standard"]["types"]>["output"];

/**
 * Vendor identifier used by all UMT validators when advertising Standard
 * Schema V1 compatibility. External tools may key off this value to attach
 * UMT-specific behavior.
 */
export const STANDARD_SCHEMA_VENDOR = "umt";

/** Minimal shape of UMT validator results consumed by `attachStandard`. */
export interface UmtValidatorResult {
  validate: boolean;
  message: string;
  type: unknown;
}

/**
 * Attaches a Standard Schema V1 `~standard` property to a UMT validator
 * function in place. The validator's existing call signature, attached
 * helpers (such as `shape` on `object()` or `implement` on `function_()`),
 * and return type are preserved untouched; only the `~standard` property is
 * added.
 *
 * Validation is delegated to the wrapped validator. On success the input
 * value is returned through the Standard Schema `value` field; on failure a
 * single issue carrying the validator's message is emitted. UMT validators
 * never transform their input, so `Input` and `Output` default to the same
 * type.
 *
 * @template Input - The input type advertised through `~standard.types`
 * @template Output - The output type advertised through `~standard.types`
 * @template F - The validator function being augmented
 * @param {F} validator - The validator function to augment
 * @returns {F & StandardSchemaV1<Input, Output>} The same function with `~standard` attached
 */
export const attachStandard = <
  Input,
  Output = Input,
  // biome-ignore lint/suspicious/noExplicitAny: validator argument shapes vary across factories
  F extends (...arguments_: any[]) => UmtValidatorResult = (
    // biome-ignore lint/suspicious/noExplicitAny: fallback signature accepts any input
    ...arguments_: any[]
  ) => UmtValidatorResult,
>(
  validator: F,
): F & StandardSchemaV1<Input, Output> => {
  const properties: StandardSchemaV1Properties<Input, Output> = {
    version: 1,
    vendor: STANDARD_SCHEMA_VENDOR,
    validate: (value: unknown): StandardSchemaV1Result<Output> => {
      const result = (
        validator as unknown as (input: unknown) => UmtValidatorResult
      )(value);
      if (result.validate) {
        return { value: value as Output };
      }
      return {
        issues: [{ message: result.message || "Validation failed" }],
      };
    },
  };
  (validator as unknown as Record<string, unknown>)["~standard"] = properties;
  return validator as F & StandardSchemaV1<Input, Output>;
};
