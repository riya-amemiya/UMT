export interface RetryOptions {
  retries?: number;
  delay?: number;
  shouldRetry?: (error: unknown) => boolean;
}

/**
 * Retries a given async function with configurable retry logic
 *
 * @template T - The return type of the function
 * @param function_ - The async function to retry
 * @param options - Configuration options for retry behavior
 * @param options.retries - Maximum number of retry attempts (default: 3)
 * @param options.delay - Delay between retries in milliseconds (default: 1000)
 * @param options.shouldRetry - Function to determine if an error should trigger a retry (default: always retry)
 * @returns Promise that resolves with the function result or rejects with the final error
 *
 * @example
 * ```typescript
 * // Basic usage with default options
 * const result = await retry(async () => {
 *   const response = await fetch('/api/data');
 *   if (!response.ok) throw new Error('Failed to fetch');
 *   return response.json();
 * });
 *
 * // Custom retry configuration
 * const result = await retry(
 *   () => riskyOperation(),
 *   {
 *     retries: 5,
 *     delay: 2000,
 *     shouldRetry: (error) => error instanceof NetworkError
 *   }
 * );
 * ```
 */
export const retry = <T>(
  function_: () => Promise<T>,
  options: RetryOptions = {},
): Promise<T> => {
  const { retries = 3, delay = 1000, shouldRetry = () => true } = options;
  const attempt = async (remainingAttempts: number): Promise<T> => {
    try {
      return await function_();
    } catch (error) {
      if (remainingAttempts <= 0 || !shouldRetry(error)) {
        throw error;
      }
      return await new Promise((resolve) =>
        setTimeout(() => resolve(attempt(remainingAttempts - 1)), delay),
      );
    }
  };
  return attempt(retries);
};
