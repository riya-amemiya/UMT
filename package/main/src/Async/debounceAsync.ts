export interface DebouncedAsyncFunction<A extends unknown[], R> {
  (...arguments_: A): Promise<R>;
  cancel: () => void;
}

/**
 * Creates a debounced async function. Subsequent calls within `wait` ms reset
 * the timer and share a single resolution; the latest arguments win.
 *
 * @template A - Argument tuple type
 * @template R - Resolved value type
 * @param {(...args: A) => Promise<R>} function_ - Async function to debounce
 * @param {number} wait - Debounce window in milliseconds
 * @returns {DebouncedAsyncFunction<A, R>} Debounced wrapper with cancel support
 * @example
 * const search = debounceAsync(query, 300);
 * await search("foo");
 */
export const debounceAsync = <A extends unknown[], R>(
  function_: (...arguments_: A) => Promise<R>,
  wait: number,
): DebouncedAsyncFunction<A, R> => {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let pendingResolvers: ((value: R) => void)[] = [];
  let pendingRejecters: ((reason: unknown) => void)[] = [];

  const flushPromises = (): {
    resolvers: ((value: R) => void)[];
    rejecters: ((reason: unknown) => void)[];
  } => {
    const resolvers = pendingResolvers;
    const rejecters = pendingRejecters;
    pendingResolvers = [];
    pendingRejecters = [];
    return { resolvers, rejecters };
  };

  const debounced = ((...arguments_: A): Promise<R> => {
    if (timer) {
      clearTimeout(timer);
    }
    return new Promise<R>((resolve, reject) => {
      pendingResolvers.push(resolve);
      pendingRejecters.push(reject);
      timer = setTimeout(() => {
        timer = undefined;
        const { resolvers, rejecters } = flushPromises();
        function_(...arguments_).then(
          (value) => {
            for (const resolver of resolvers) {
              resolver(value);
            }
          },
          (error) => {
            for (const rejecter of rejecters) {
              rejecter(error);
            }
          },
        );
      }, wait);
    });
  }) as DebouncedAsyncFunction<A, R>;

  debounced.cancel = (): void => {
    if (timer) {
      clearTimeout(timer);
      timer = undefined;
    }
    const { rejecters } = flushPromises();
    for (const rejecter of rejecters) {
      rejecter(new Error("debounceAsync cancelled"));
    }
  };

  return debounced;
};
