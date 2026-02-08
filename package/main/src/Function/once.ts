/**
 * Creates a function that is restricted to be called only once.
 * Subsequent calls return the result of the first invocation.
 *
 * @param function_ - The function to restrict.
 * @returns A function that invokes the original only on its first call.
 *
 * @example
 * ```typescript
 * const initialize = once(() => {
 *   console.log("initialized");
 *   return 42;
 * });
 * initialize(); // logs "initialized", returns 42
 * initialize(); // returns 42 (no log)
 * ```
 */
export const once = <A extends unknown[], R>(
  function_: (...arguments_: A) => R,
): ((...arguments_: A) => R) => {
  let called = false;
  let result: R;

  return function (this: unknown, ...arguments_: A): R {
    if (!called) {
      called = true;
      result = function_.apply(this, arguments_);
    }
    return result;
  };
};
