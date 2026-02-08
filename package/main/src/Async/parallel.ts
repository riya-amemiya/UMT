/**
 * Executes async functions in parallel with a concurrency limit
 * @param {number} limit - Maximum number of concurrent executions
 * @param {T[]} items - The items to process
 * @param {(item: T, index: number) => Promise<U>} function_ - The async function to apply to each item
 * @returns {Promise<U[]>} Results in the same order as the input items
 * @example
 * const results = await parallel(2, [1, 2, 3], async (n) => n * 2);
 */
export const parallel = <T, U>(
  limit: number,
  items: T[],
  function_: (item: T, index: number) => Promise<U>,
): Promise<U[]> => {
  const results: U[] = Array.from({ length: items.length });
  let nextIndex = 0;
  let resolvedCount = 0;

  return new Promise<U[]>((resolve, reject) => {
    if (items.length === 0) {
      resolve(results);
      return;
    }

    const runNext = (): void => {
      if (nextIndex >= items.length) {
        return;
      }
      const currentIndex = nextIndex;
      nextIndex += 1;

      function_(items[currentIndex], currentIndex)
        .then((result) => {
          results[currentIndex] = result;
          resolvedCount += 1;

          if (resolvedCount === items.length) {
            resolve(results);
          } else {
            runNext();
          }
        })
        .catch(reject);
    };

    const initialBatch = Math.min(limit, items.length);
    for (let index = 0; index < initialBatch; index += 1) {
      runNext();
    }
  });
};
