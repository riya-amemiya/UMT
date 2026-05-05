/**
 * String validation module for literal union (allowed values) check
 * Provides validation functionality for checking if a string is one of the
 * allowed literal values, useful for validating string literal union types
 * such as `'standard' | 'squat' | 'decanter' | 'round' | 'tall' | 'flask'`.
 *
 * The validator's return type carries the literal union through its `type`
 * field directly (instead of going through `Types<T>` which would collapse
 * to `"string"`), so consumers like `object()`, `union()`, and `intersection()`
 * can preserve the literal union in their inferred types.
 */

/**
 * Return type produced by a `oneOf` validator. Structurally compatible with
 * `ValidateCoreReturnType<unknown>`, but exposes the literal union directly
 * via the `type` field so the inferred type can flow through `object()`,
 * `union()`, and `intersection()` without being collapsed to `string`.
 */
export interface OneOfReturnType<T extends string> {
  validate: boolean;
  message: string;
  type: T;
}

/**
 * Creates a top-level validator that checks if a string value is one of the
 * given allowed literal values. The literal union is captured via `const T`
 * and exposed through the `OneOfReturnType<T[number]>` return type.
 * @template T - The tuple of allowed string literals
 * @param {T} values - The tuple of allowed string values
 * @param {string} [message] - Custom error message for validation failure
 * @returns A validator function from `string` to `OneOfReturnType<T[number]>`
 */
export const oneOf = <const T extends readonly string[]>(
  values: T,
  message?: string,
) => {
  const allowed = new Set<string>(values);
  return (value: string): OneOfReturnType<T[number]> => {
    const isValid = allowed.has(value);
    return {
      validate: isValid,
      message: isValid ? "" : (message ?? ""),
      type: value,
    };
  };
};
