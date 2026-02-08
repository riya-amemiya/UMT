import type { Result } from "@/Error/safeExecute";

/**
 * Transforms the value inside a successful Result using a function
 * that itself returns a Result. If the original Result is an error,
 * it is returned unchanged.
 *
 * @param result - The Result to transform.
 * @param function_ - The function to apply to the success value,
 *   which returns a new Result.
 * @returns The Result returned by the mapping function, or the original error.
 *
 * @example
 * ```typescript
 * const success = { type: "success", value: 5 } as const;
 * flatMapResult(success, (n) =>
 *   n > 0
 *     ? { type: "success", value: n * 2 }
 *     : { type: "error", error: new Error("negative") }
 * );
 * // { type: "success", value: 10 }
 * ```
 */
export const flatMapResult = <V, E, U, F>(
  result: Result<V, E>,
  function_: (value: V) => Result<U, F>,
): Result<U, E | F> => {
  if (result.type === "success") {
    return function_(result.value);
  }
  return result;
};
