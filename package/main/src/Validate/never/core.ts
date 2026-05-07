/**
 * Never validation core module
 * Provides a validator that fails for every input, useful for marking
 * positions in a schema that must never be filled (for example, exhaustive
 * union members that should be unreachable at the type level).
 */

/**
 * Return type produced by a `never` validator. Exposes the literal `"never"`
 * tag through the `type` field so `ValidateType<"never">` can map it back to
 * the `never` runtime type when consumed by downstream helpers.
 */
export interface NeverReturnType {
  validate: boolean;
  message: string;
  type: "never";
}

/**
 * Creates a validator that always fails
 * @param {string} [message] - Custom error message for validation failure
 * @returns {Function} - Validator that always returns a failing result
 */
// biome-ignore lint/suspicious/noExplicitAny: never() is widened to accept any input from union/intersection
export const never = (message?: string): ((value: any) => NeverReturnType) => {
  // biome-ignore lint/suspicious/noExplicitAny: signature mirrors the public type
  return (_value: any): NeverReturnType => {
    return {
      validate: false,
      message: message ?? "",
      type: "never",
    };
  };
};
