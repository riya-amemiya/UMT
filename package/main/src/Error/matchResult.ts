import type { Result } from "@/Error/safeExecute";

/**
 * Pattern-matches on a Result, applying the appropriate handler
 * depending on whether the Result is a success or an error.
 *
 * @param result - The Result to match on.
 * @param handlers - An object with onSuccess and onError callbacks.
 * @param handlers.onSuccess - Called with the value if the Result is a success.
 * @param handlers.onError - Called with the error if the Result is an error.
 * @returns The return value of whichever handler is invoked.
 *
 * @example
 * ```typescript
 * const result = { type: "success", value: 42 } as const;
 * matchResult(result, {
 *   onSuccess: (v) => `Got ${v}`,
 *   onError: (e) => `Failed: ${e}`,
 * });
 * // "Got 42"
 * ```
 */
export const matchResult = <V, E, S, F>(
  result: Result<V, E>,
  handlers: {
    onSuccess: (value: V) => S;
    onError: (error: E) => F;
  },
): S | F => {
  if (result.type === "success") {
    return handlers.onSuccess(result.value);
  }
  return handlers.onError(result.error);
};
