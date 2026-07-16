import {
  errorFunction,
  type Result,
  successFunction,
} from "@/Error/safeExecute";

/**
 * Safely executes an async callback and returns a Result type
 * Catches any errors and wraps them in a Result type
 * @template V The type of the success value
 * @template E The type of the error (defaults to Error)
 * @param callback The async (or sync) function to execute safely
 * @returns A Promise of a Result containing either the value or the error
 * @example
 * await safeExecuteAsync(async () => 42);
 * // { type: "success", value: 42 }
 */
export const safeExecuteAsync = async <V, E = Error>(
  callback: () => Promise<V> | V,
): Promise<Result<V, E>> => {
  try {
    return successFunction<V>(await callback());
  } catch (error) {
    return errorFunction<E>(error as E);
  }
};
