import { compareFunctionDefault } from "./compareFunctionDefault";

export const insertionSort = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length - 1,
): T[] => {
  for (let index = start + 1; index <= end; index++) {
    let index_ = index;
    const target = array[index];
    while (index_ > start && compareFunction(array[index_ - 1], target) > 0) {
      array[index_] = array[index_ - 1];
      index_--;
    }
    array[index_] = target;
  }
  return array;
};
