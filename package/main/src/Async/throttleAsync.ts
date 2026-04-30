export interface ThrottledAsyncFunction<A extends unknown[], R> {
  (...arguments_: A): Promise<R>;
  cancel: () => void;
}

/**
 * Creates a throttled async function. Coalesces concurrent calls within the
 * `wait` window so only one underlying invocation runs; all callers in the
 * window receive the same result.
 *
 * @template A - Argument tuple type
 * @template R - Resolved value type
 * @param {(...args: A) => Promise<R>} function_ - Async function to throttle
 * @param {number} wait - Window length in milliseconds
 * @returns {ThrottledAsyncFunction<A, R>} Throttled wrapper with cancel support
 * @example
 * const fetchUser = throttleAsync(loadUser, 1000);
 */
export const throttleAsync = <A extends unknown[], R>(
  function_: (...arguments_: A) => Promise<R>,
  wait: number,
): ThrottledAsyncFunction<A, R> => {
  let inflight: Promise<R> | undefined;
  let lockedUntil = 0;

  const throttled = ((...arguments_: A): Promise<R> => {
    if (inflight) {
      return inflight;
    }
    if (Date.now() < lockedUntil) {
      return Promise.reject(new Error("throttleAsync window not elapsed"));
    }
    const current = function_(...arguments_);
    inflight = current;
    const cleanup = (): void => {
      if (inflight === current) {
        inflight = undefined;
        lockedUntil = Date.now() + wait;
      }
    };
    current.then(cleanup, cleanup);
    return current;
  }) as ThrottledAsyncFunction<A, R>;

  throttled.cancel = (): void => {
    inflight = undefined;
    lockedUntil = 0;
  };

  return throttled;
};
