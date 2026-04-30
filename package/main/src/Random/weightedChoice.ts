export interface WeightedItem<T> {
  value: T;
  weight: number;
}

/**
 * Returns a random element using cumulative weights and binary search.
 * Items with non-positive weights are skipped from the pool.
 *
 * @template T - Element type
 * @param {readonly WeightedItem<T>[]} items - Items with non-negative weights
 * @returns {T} A randomly selected element
 * @example
 * weightedChoice([{ value: "a", weight: 1 }, { value: "b", weight: 4 }]);
 */
export const weightedChoice = <T>(items: readonly WeightedItem<T>[]): T => {
  const cumulative: number[] = [];
  let total = 0;
  for (const item of items) {
    if (item.weight > 0) {
      total += item.weight;
    }
    cumulative.push(total);
  }
  const target = Math.random() * total;
  let lo = 0;
  let hi = cumulative.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >>> 1;
    if (cumulative[mid] <= target) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return items[lo].value;
};
