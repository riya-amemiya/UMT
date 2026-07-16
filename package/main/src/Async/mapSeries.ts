/**
 * Maps over items sequentially, awaiting each async function before the next
 * @param {T[]} items The items to process
 * @param {(item: T, index: number) => Promise<U>} function_ The async mapper
 * @returns {Promise<U[]>} Results in the same order as the input items
 * @example
 * const results = await mapSeries([1, 2, 3], async (n) => n * 2);
 * // [2, 4, 6]
 */
export const mapSeries = async <T, U>(
  items: T[],
  function_: (item: T, index: number) => Promise<U>,
): Promise<U[]> => {
  const length = items.length;
  const results: U[] = new Array(length);
  for (let index = 0; index < length; index++) {
    // Sequential await is intentional: mapSeries must not overlap tasks.
    // biome-ignore lint/performance/noAwaitInLoops: sequential by design
    results[index] = await function_(items[index], index);
  }
  return results;
};
