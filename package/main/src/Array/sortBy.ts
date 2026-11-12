import { compareFunctionDefault } from "./compareFunctionDefault";
import { timSort } from "./timSort";

/**
Sorts an array by one or more key selectors with a stable TimSort
Each selector is called once per element. Later selectors break ties left by
earlier ones, and elements whose keys are all equal keep their original order.
Keys are compared with compareFunctionDefault, so incomparable keys such as
NaN compare as equal.
@param {T[]} array The array to sort (not mutated)
@param {((value: T) => unknown)[]} selectors
  Functions returning the sort keys, in priority order
@returns {T[]} A new sorted array
@example sortBy([{ age: 30 }, { age: 20 }], (user) => user.age); // [{ age: 20 }, { age: 30 }]
@example sortBy(["bb", "a", "ab"], (s) => s.length, (s) => s); // ["a", "ab", "bb"]
*/
export const sortBy = <T>(
  array: T[],
  ...selectors: ((value: T) => unknown)[]
): T[] => {
  const entries = array.map((value) => ({
    keys: selectors.map((selector) => selector(value)),
    value,
  }));
  const sorted = timSort(entries, (a, b) => {
    for (const [index, key] of a.keys.entries()) {
      const order = compareFunctionDefault(key, b.keys[index]);
      if (order !== 0) {
        return order;
      }
    }
    return 0;
  });
  return sorted.map((entry) => entry.value);
};
