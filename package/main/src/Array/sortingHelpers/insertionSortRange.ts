import type { CompareFunction } from "$/array/compareFunction";

/**
 * @internal
 */
export const insertionSortRange = <T>(
  array: T[],
  compareFunction: CompareFunction<T>,
  start: number,
  end: number,
): void => {
  for (let index = start + 1; index <= end; index++) {
    let index_ = index;
    const target = array[index];
    while (index_ > start && compareFunction(array[index_ - 1], target) > 0) {
      array[index_] = array[index_ - 1];
      index_--;
    }
    array[index_] = target;
  }
};
