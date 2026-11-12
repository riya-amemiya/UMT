/**
Options for the debounce function.
*/
export interface DebounceOptions {
  /**
  Whether to invoke on the leading edge of the timeout.
  */
  leading?: boolean;
  /**
  Whether to invoke on the trailing edge of the timeout.
  */
  trailing?: boolean;
}

/**
A debounced function with cancel, flush and pending methods.
*/
export interface DebouncedFunction<
  T extends (...arguments_: unknown[]) => unknown,
> {
  (...arguments_: Parameters<T>): void;
  /**
  Cancels any pending debounced invocation.
  */
  cancel: () => void;
  /**
  Immediately runs the pending trailing invocation, if any, and clears the
  timer. Does nothing when no timer is pending.
  */
  flush: () => void;
  /**
  Returns whether a debounce timer is currently pending.
  */
  pending: () => boolean;
}

/**
Creates a debounced version of the provided function that delays
invoking it until after the specified wait time has elapsed since
the last time the debounced function was called.

@param function_ - The function to debounce.
@param wait - The number of milliseconds to delay.
@param options - Configuration for leading/trailing invocation.
@param options.leading - If true, invoke on the leading edge.
@param options.trailing - If true, invoke on the trailing edge (default).
@returns The debounced function with cancel, flush and pending methods.

@example
```typescript
const debounced = debounce(() => console.log("called"), 300);
debounced();
debounced();
debounced.pending(); // true
debounced.flush(); // logs "called" immediately
debounced.pending(); // false
debounced();
debounced.cancel();
```
*/
export const debounce = <T extends (...arguments_: unknown[]) => unknown>(
  function_: T,
  wait: number,
  options: DebounceOptions = {},
): DebouncedFunction<T> => {
  const { leading: isLeading = false, trailing: isTrailing = true } = options;
  let timerId: ReturnType<typeof setTimeout> | undefined;
  let lastArguments: Parameters<T> | undefined;
  const thisReference = { value: undefined as unknown };
  let lastCallTime = 0;

  const trailingEdge = () => {
    timerId = undefined;
    if (!isTrailing || lastArguments === undefined) {
      return;
    }

    function_.apply(thisReference.value, lastArguments);
    lastArguments = undefined;
  };

  const scheduleCheck = () => {
    timerId = setTimeout(
      () => {
        const elapsed = Date.now() - lastCallTime;
        const remaining = wait - elapsed;
        if (remaining <= 0) {
          trailingEdge();
        } else {
          scheduleCheck();
        }
      },
      wait - (Date.now() - lastCallTime),
    );
  };

  const debounced = function (
    this: unknown,
    ...arguments_: Parameters<T>
  ): void {
    lastArguments = arguments_;
    thisReference.value = this;
    lastCallTime = Date.now();

    const isFirstCall = timerId === undefined;

    if (isLeading && isFirstCall) {
      function_.apply(thisReference.value, lastArguments);
      lastArguments = undefined;
    }

    if (isFirstCall) {
      scheduleCheck();
    }
  } as DebouncedFunction<T>;

  debounced.cancel = () => {
    if (timerId !== undefined) {
      clearTimeout(timerId);
    }
    timerId = undefined;
    lastArguments = undefined;
    thisReference.value = undefined;
    lastCallTime = 0;
  };

  debounced.flush = () => {
    if (timerId === undefined) {
      return;
    }
    clearTimeout(timerId);
    trailingEdge();
  };

  debounced.pending = () => timerId !== undefined;

  return debounced;
};
