import type { CompareFunction } from "$/array/compareFunction";

/**
 * Three-way partitioning (Dutch National Flag)
 * Returns [lt, gt] where:
 * - elements < pivot are in [low, lt)
 * - elements = pivot are in [lt, gt]
 * - elements > pivot are in (gt, high]
 */
export const partition3Way = <T>(
  array: T[],
  low: number,
  high: number,
  pivotIndex: number,
  compareFunction: CompareFunction<T>,
): [number, number] => {
  const pivot = array[pivotIndex];
  let index = low;
  let lt = low;
  let gt = high;

  while (index <= gt) {
    const cmp = compareFunction(array[index], pivot);
    if (cmp < 0) {
      [array[index], array[lt]] = [array[lt], array[index]];
      index++;
      lt++;
    } else if (cmp > 0) {
      [array[index], array[gt]] = [array[gt], array[index]];
      gt--;
    } else {
      index++;
    }
  }

  return [lt, gt];
};
