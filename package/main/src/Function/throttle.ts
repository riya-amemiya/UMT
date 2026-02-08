/**
 * A throttled function with a cancel method.
 */
export interface ThrottledFunction<
  T extends (...arguments_: unknown[]) => unknown,
> {
  (...arguments_: Parameters<T>): void;
  /** Cancels any pending throttled invocation. */
  cancel: () => void;
}

/**
 * Creates a throttled version of the provided function that only
 * invokes it at most once per the specified wait period.
 *
 * @param function_ - The function to throttle.
 * @param wait - The minimum time between invocations in milliseconds.
 * @returns The throttled function with a cancel method.
 *
 * @example
 * ```typescript
 * const throttled = throttle(() => console.log("called"), 300);
 * throttled();
 * throttled();
 * throttled.cancel();
 * ```
 */
export const throttle = <T extends (...arguments_: unknown[]) => unknown>(
  function_: T,
  wait: number,
): ThrottledFunction<T> => {
  let lastCallTime = 0;
  let timerId: ReturnType<typeof setTimeout> | undefined;
  let lastContext: { arguments_: Parameters<T>; thisArg: unknown } | undefined;

  const invoke = () => {
    lastCallTime = Date.now();
    const context = lastContext as {
      arguments_: Parameters<T>;
      thisArg: unknown;
    };
    function_.apply(context.thisArg, context.arguments_);
    lastContext = undefined;
  };

  const throttled = function (
    this: unknown,
    ...arguments_: Parameters<T>
  ): void {
    lastContext = { arguments_, thisArg: this };
    const elapsed = Date.now() - lastCallTime;
    const remaining = wait - elapsed;

    if (remaining <= 0) {
      if (timerId !== undefined) {
        clearTimeout(timerId);
        timerId = undefined;
      }
      invoke();
    } else {
      timerId ??= setTimeout(() => {
        timerId = undefined;
        invoke();
      }, remaining);
    }
  } as ThrottledFunction<T>;

  throttled.cancel = () => {
    if (timerId !== undefined) {
      clearTimeout(timerId);
    }
    timerId = undefined;
    lastContext = undefined;
    lastCallTime = 0;
  };

  return throttled;
};
