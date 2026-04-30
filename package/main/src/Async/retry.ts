import { sleep } from "./sleep";

export interface RetryOptions {
  retries?: number;
  delay?: number;
  backoff?: "fixed" | "exponential" | "linear";
  jitter?: boolean;
  shouldRetry?: (error: unknown, attempt: number) => boolean;
  onRetry?: (error: unknown, attempt: number) => void;
  signal?: AbortSignal;
}

const computeDelay = (
  baseDelay: number,
  attemptNumber: number,
  backoff: NonNullable<RetryOptions["backoff"]>,
  jitter: boolean,
): number => {
  let waitMs = baseDelay;
  if (backoff === "linear") {
    waitMs = baseDelay * attemptNumber;
  } else if (backoff === "exponential") {
    waitMs = baseDelay * 2 ** (attemptNumber - 1);
  }
  if (jitter) {
    waitMs *= Math.random();
  }
  return waitMs;
};

/**
 * Retries an async function until it succeeds or the retry budget is exhausted.
 * Supports fixed, linear, or exponential backoff, optional jitter, and
 * AbortSignal cancellation.
 *
 * @template T - Return type of the function
 * @param {() => Promise<T>} function_ - The async function to invoke
 * @param {RetryOptions} [options] - Retry configuration
 * @returns {Promise<T>} Result of the first successful invocation
 * @example
 * await retry(() => fetch("/api"), { retries: 5, backoff: "exponential", jitter: true });
 */
export const retry = async <T>(
  function_: () => Promise<T>,
  options: RetryOptions = {},
): Promise<T> => {
  const {
    retries = 3,
    delay = 1000,
    backoff = "fixed",
    jitter = false,
    shouldRetry = () => true,
    onRetry,
    signal,
  } = options;

  let attemptNumber = 0;
  while (true) {
    if (signal?.aborted) {
      throw signal.reason ?? new Error("Aborted");
    }
    try {
      // biome-ignore lint/performance/noAwaitInLoops: retry must await sequentially
      return await function_();
    } catch (error) {
      const remaining = retries - attemptNumber;
      if (remaining <= 0 || !shouldRetry(error, attemptNumber)) {
        throw error;
      }
      onRetry?.(error, attemptNumber);
      await sleep(computeDelay(delay, attemptNumber + 1, backoff, jitter));
      attemptNumber += 1;
    }
  }
};
