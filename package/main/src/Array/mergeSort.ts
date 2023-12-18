import { compareFunctionDefault } from "./compareFunctionDefault";

export const mergeSort = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length,
): T[] => {
  const length = end - start;
  if (length <= 1) {
    return array.slice(start, end);
  }

  const middle = start + Math.floor(length / 2);
  const left = mergeSort(array, compareFunction, start, middle);
  const right = mergeSort(array, compareFunction, middle, end);

  return merge(left, right, compareFunction);
};

function merge<T>(
  left: T[],
  right: T[],
  compareFunction: (a: T, b: T) => number,
): T[] {
  const result: T[] = [];
  let leftIndex = 0;
  let rightIndex = 0;

  while (leftIndex < left.length && rightIndex < right.length) {
    if (compareFunction(left[leftIndex], right[rightIndex]) <= 0) {
      result.push(left[leftIndex]);
      leftIndex++;
    } else {
      result.push(right[rightIndex]);
      rightIndex++;
    }
  }

  return result.concat(left.slice(leftIndex)).concat(right.slice(rightIndex));
}
