import type { CompareFunction } from "$/array/compareFunction";

/**
@internal
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
      index_--;
    }
    if (index_ === index) {
      continue;
    }
    array.copyWithin(index_ + 1, index_, index);
    array[index_] = target;
  }
};
