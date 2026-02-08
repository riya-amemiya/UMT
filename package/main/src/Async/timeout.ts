/**
 * Wraps a promise with a timeout, rejecting if it does not
 * resolve within the specified milliseconds
 * @param {Promise<T>} promise - The promise to wrap
 * @param {number} ms - The timeout in milliseconds
 * @returns {Promise<T>} The result of the promise or a timeout error
 * @example
 * const result = await timeout(fetch("/api"), 5000);
 */
export const timeout = <T>(promise: Promise<T>, ms: number): Promise<T> =>
  new Promise<T>((resolve, reject) => {
    const timer = setTimeout(() => {
      reject(new Error(`Timed out after ${ms}ms`));
    }, ms);

    promise.then(
      (value) => {
        clearTimeout(timer);
        resolve(value);
      },
      (error) => {
        clearTimeout(timer);
        reject(error);
      },
    );
  });
