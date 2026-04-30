export type SettledResult<T> =
  | { status: "fulfilled"; value: T }
  | { status: "rejected"; reason: unknown };

/**
 * Awaits all promises and returns their settled results, with an optional
 * concurrency limit applied during execution.
 *
 * @template T - Resolved value type
 * @param {Iterable<Promise<T> | (() => Promise<T>)>} tasks - Promises or thunks
 * @param {number} [limit] - Maximum concurrent in-flight tasks; unlimited when omitted
 * @returns {Promise<SettledResult<T>[]>} Settled results in input order
 * @example
 * await pSettled([Promise.resolve(1), Promise.reject(new Error("x"))]);
 */
export const pSettled = <T>(
  tasks: Iterable<Promise<T> | (() => Promise<T>)>,
  limit?: number,
): Promise<SettledResult<T>[]> => {
  const items = [...tasks];
  const results: SettledResult<T>[] = Array.from({ length: items.length });
  if (items.length === 0) {
    return Promise.resolve(results);
  }
  const effectiveLimit = limit && limit > 0 ? limit : items.length;
  let nextIndex = 0;
  let resolvedCount = 0;

  return new Promise<SettledResult<T>[]>((resolve) => {
    const runNext = (): void => {
      if (nextIndex >= items.length) {
        return;
      }
      const currentIndex = nextIndex;
      nextIndex += 1;
      const task = items[currentIndex];
      const promise =
        typeof task === "function"
          ? Promise.resolve().then(task)
          : Promise.resolve(task);

      promise
        .then(
          (value) => {
            results[currentIndex] = { status: "fulfilled", value };
          },
          (error) => {
            results[currentIndex] = { status: "rejected", reason: error };
          },
        )
        .then(() => {
          resolvedCount += 1;
          if (resolvedCount === items.length) {
            resolve(results);
          } else {
            runNext();
          }
        });
    };

    const initialBatch = Math.min(effectiveLimit, items.length);
    for (let index = 0; index < initialBatch; index += 1) {
      runNext();
    }
  });
};
