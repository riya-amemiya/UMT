/**
 * Represents a successful execution result containing a value
 * @template V The type of the success value
 */
interface SuccessType<V> {
  type: "success";
  value: V;
}

/**
 * Represents an error result containing an error object
 * @template E The type of the error
 */
interface ErrorType<E> {
  type: "error";
  error: E;
}

/**
 * Union type representing either a successful result or an error result
 * @template V The type of the success value
 * @template E The type of the error
 */
export type Result<V, E> = SuccessType<V> | ErrorType<E>;
/**
 * Creates an error result
 * @template E The type of the error
 * @param error The error object
 * @returns An ErrorType containing the error
 */
const errorFunction = <E>(error: E): ErrorType<E> => ({ type: "error", error });
/**
 * Creates a success result
 * @template V The type of the success value
 * @param value The success value
 * @returns A SuccessType containing the value
 */
const successFunction = <V>(value: V): SuccessType<V> => ({
  type: "success",
  value,
});
/**
 * Safely executes a callback function and returns a Result type
 * Catches any errors and wraps them in a Result type
 * @template V The type of the success value
 * @template E The type of the error (defaults to Error)
 * @param callback The function to execute safely
 * @returns A Result containing either the successful value or the caught error
 */
export const safeExecute = <V, E = Error>(callback: () => V): Result<V, E> => {
  try {
    return successFunction<V>(callback());
  } catch (error) {
    return errorFunction<E>(error as E);
  }
};
