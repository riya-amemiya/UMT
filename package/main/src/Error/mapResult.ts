import { type Result, successFunction } from "@/Error/safeExecute";

/**
 * Transforms the value inside a successful Result using the provided
 * mapping function. If the Result is an error, it is returned unchanged.
 *
 * @param result - The Result to transform.
 * @param function_ - The function to apply to the success value.
 * @returns A new Result with the transformed value, or the original error.
 *
 * @example
 * ```typescript
 * const success = { type: "success", value: 5 } as const;
 * mapResult(success, (n) => n * 2);
 * // { type: "success", value: 10 }
 *
 * const error = { type: "error", error: new Error("fail") } as const;
 * mapResult(error, (n) => n * 2);
 * // { type: "error", error: Error("fail") }
 * ```
 */
export const mapResult = <V, E, U>(
  result: Result<V, E>,
  function_: (value: V) => U,
): Result<U, E> => {
  if (result.type === "success") {
    return successFunction(function_(result.value));
  }
  return result;
};
