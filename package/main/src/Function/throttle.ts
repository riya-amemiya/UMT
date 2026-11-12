/**
A throttled function with cancel, flush and pending methods.
*/
export interface ThrottledFunction<
  T extends (...arguments_: unknown[]) => unknown,
> {
  (...arguments_: Parameters<T>): void;
  /**
  Cancels any pending throttled invocation.
  */
  cancel: () => void;
  /**
  Immediately runs the pending trailing invocation, if any, and clears the
  timer. Does nothing when no trailing invocation is pending.
  */
  flush: () => void;
  /**
  Returns whether a trailing throttled invocation is currently pending.
  */
  pending: () => boolean;
}

/**
Creates a throttled version of the provided function that only
invokes it at most once per the specified wait period.

@param function_ - The function to throttle.
@param wait - The minimum time between invocations in milliseconds.
@returns The throttled function with cancel, flush and pending methods.

@example
```typescript
const throttled = throttle(() => console.log("called"), 300);
throttled(); // logs "called"
throttled();
throttled.pending(); // true
throttled.flush(); // logs "called" immediately
throttled.pending(); // false
throttled();
throttled.cancel();
```
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

  throttled.flush = () => {
    if (timerId === undefined) {
      return;
    }
    clearTimeout(timerId);
    timerId = undefined;
    invoke();
  };

  throttled.pending = () => timerId !== undefined;

  return throttled;
};
