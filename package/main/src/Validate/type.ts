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

/**
 * Maps TypeScript types to their string literal representations
 * @template T - The type to map
 * @returns "string" for string, "number" for number, "boolean" for boolean, or the original type T otherwise
 */
export type Types<T> = T extends string | number | boolean
  ? _Types<T>
  : T extends undefined | null
    ? _Types2<T>
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

/**
 * Maps string literal type names back to their TypeScript types
 * @template T - The string literal type name ("string", "number", "boolean")
 * @returns The corresponding TypeScript type (string, number, boolean) or the original type T
 */
export type ValidateType<T> = T extends "string" | "number" | "boolean"
  ? _ValidateType<T>
  : T extends "undefined" | "null"
    ? _ValidateType2<T>
    : T;

export type SchemaToInterface<
  // biome-ignore lint/suspicious/noExplicitAny: ignore
  T extends (value: any) => ValidateCoreReturnType<any>,
> = ReturnType<T>["type"];

export type OptionalKeys<T> = {
  [K in keyof T]: undefined extends T[K] ? K : never;
}[keyof T];
