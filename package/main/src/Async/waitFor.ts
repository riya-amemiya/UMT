import { sleep } from "./sleep";

export interface WaitForOptions {
  timeout?: number;
  interval?: number;
  signal?: AbortSignal;
}

/**
 * Repeatedly evaluates a condition until it returns a truthy value or the
 * timeout elapses. Resolves with the final truthy value.
 *
 * @template T - Truthy result type
 * @param {() => T | Promise<T>} condition - Condition predicate
 * @param {WaitForOptions} [options] - Polling configuration
 * @returns {Promise<NonNullable<T>>} The first truthy value
 * @throws {Error} When the timeout elapses or the signal aborts
 * @example
 * await waitFor(() => document.querySelector("#root"), { interval: 100 });
 */
export const waitFor = async <T>(
  condition: () => T | Promise<T>,
  options: WaitForOptions = {},
): Promise<NonNullable<T>> => {
  const { timeout = 5000, interval = 100, signal } = options;
  const startTime = Date.now();

  while (true) {
    if (signal?.aborted) {
      throw signal.reason ?? new Error("Aborted");
    }
    // biome-ignore lint/performance/noAwaitInLoops: waitFor polls sequentially
    const value = await condition();
    if (value) {
      return value;
    }
    if (Date.now() - startTime >= timeout) {
      throw new Error(`waitFor timed out after ${timeout}ms`);
    }
    await sleep(interval);
  }
};
