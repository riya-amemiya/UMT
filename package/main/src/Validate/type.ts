/**
 * Maps TypeScript types to their string literal representations
 * @template T - The type to map
 * @returns "string" for string, "number" for number, "boolean" for boolean, or the original type T otherwise
 */
export type Types<T> = T extends string
  ? "string"
  : T extends number
    ? "number"
    : T extends boolean
      ? "boolean"
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

/**
 * Maps string literal type names back to their TypeScript types
 * @template T - The string literal type name ("string", "number", "boolean")
 * @returns The corresponding TypeScript type (string, number, boolean) or the original type T
 */
export type ValidateType<T> = T extends "string"
  ? string
  : T extends "number"
    ? number
    : T extends "boolean"
      ? boolean
      : T;

export type SchemaToInterface<
  // biome-ignore lint/suspicious/noExplicitAny: ignore
  T extends (value: any) => ValidateCoreReturnType<any>,
> = ReturnType<T>["type"];
