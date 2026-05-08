// TS can only infer deeply with up to 3 extends, so we split it to avoid the limit
export type _Types<T> = T extends string
  ? "string"
  : T extends number
    ? "number"
    : T extends boolean
      ? "boolean"
      : T;

export type _Types2<T> = T extends undefined
  ? "undefined"
  : T extends null
    ? "null"
    : T;

// TS can only infer deeply with up to 3 extends, so we split it to avoid the limit
export type _Types3<T> = T extends bigint ? "bigint" : T;

/**
 * Maps TypeScript types to their string literal representations
 * @template T - The type to map
 * @returns "string" for string, "number" for number, "boolean" for boolean, "bigint" for bigint, or the original type T otherwise
 */
export type Types<T> = T extends string | number | boolean
  ? _Types<T>
  : T extends undefined | null
    ? _Types2<T>
    : T extends bigint
      ? _Types3<T>
      : T;

/**
 * Core validation result type including validation status, message, and type information
 * @template T - The type being validated
 */
export interface ValidateCoreReturnType<T> {
  validate: boolean;
  message: string;
  type: Types<T>;
}

/**
 * Extended validation result type including type information and validation function
 * @template T - The type being validated
 */
export interface ValidateReturnType<T> {
  type: Types<T>;
  validate: ValidateFunctionType<T>;
  message?: string;
}

/**
 * Type for validation functions that take a value and return a boolean
 * @template T - The type of value to validate
 */
export type ValidateFunctionType<T> = (value: T) => boolean;

declare const LITERAL_BRAND: unique symbol;

/**
 * Branded literal type used by validators (such as `oneOf`) that need to keep
 * a string literal verbatim through `ValidateType`. The brand prevents the
 * literal from colliding with reserved type tags like `"string"`, `"number"`,
 * or `"boolean"` when the same string also happens to name a primitive type.
 * @template T - The literal value preserved by the brand
 */
export type LiteralBrand<T extends string> = T & {
  readonly [LITERAL_BRAND]: T;
};

// TS can only infer deeply with up to 3 extends, so we split it to avoid the limit
export type _ValidateType<T> = T extends "string"
  ? string
  : T extends "number"
    ? number
    : T extends "boolean"
      ? boolean
      : T;

export type _ValidateType2<T> = T extends "undefined"
  ? undefined
  : T extends "null"
    ? null
    : T;

// TS can only infer deeply with up to 3 extends, so we split it to avoid the limit
export type _ValidateType3<T> = T extends "bigint"
  ? bigint
  : T extends "any"
    ? // biome-ignore lint/suspicious/noExplicitAny: tag must map back to any
      any
    : T extends "unknown"
      ? unknown
      : T extends "never"
        ? never
        : T;

/**
 * Maps string literal type names back to their TypeScript types
 * @template T - The string literal type name ("string", "number", "boolean", "bigint", "undefined", "null", "any", "unknown", "never")
 * @returns The corresponding TypeScript type, the unwrapped literal when `T` is a `LiteralBrand`, or the original type `T` when no tag matches
 */
export type ValidateType<T> =
  T extends LiteralBrand<infer L>
    ? L
    : T extends "string" | "number" | "boolean"
      ? _ValidateType<T>
      : T extends "undefined" | "null"
        ? _ValidateType2<T>
        : T extends "bigint" | "any" | "unknown" | "never"
          ? _ValidateType3<T>
          : T;

export type SchemaToInterface<
  // biome-ignore lint/suspicious/noExplicitAny: ignore
  T extends (value: any) => ValidateCoreReturnType<any>,
> = ValidateType<ReturnType<T>["type"]>;

export type OptionalKeys<T> = {
  [K in keyof T]: undefined extends T[K] ? K : never;
}[keyof T];
