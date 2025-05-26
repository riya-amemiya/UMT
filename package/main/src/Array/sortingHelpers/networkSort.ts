import type { CompareFunction } from "$/array/compareFunction";

/**
 * Optimized network sort for arrays up to 5 elements
 * Uses hardcoded comparison sequences for minimal overhead
 */
export const networkSort = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): void => {
  const length = high - low + 1;

  if (length === 2) {
    if (compareFunction(array[low], array[high]) > 0) {
      [array[low], array[high]] = [array[high], array[low]];
    }
    return;
  }

  if (length === 3) {
    // 3-element sorting network
    if (compareFunction(array[low], array[low + 1]) > 0) {
      [array[low], array[low + 1]] = [array[low + 1], array[low]];
    }
    if (compareFunction(array[low + 1], array[high]) > 0) {
      [array[low + 1], array[high]] = [array[high], array[low + 1]];
    }
    if (compareFunction(array[low], array[low + 1]) > 0) {
      [array[low], array[low + 1]] = [array[low + 1], array[low]];
    }
    return;
  }

  if (length === 4) {
    // 4-element sorting network
    if (compareFunction(array[low], array[low + 2]) > 0) {
      [array[low], array[low + 2]] = [array[low + 2], array[low]];
    }
    if (compareFunction(array[low + 1], array[high]) > 0) {
      [array[low + 1], array[high]] = [array[high], array[low + 1]];
    }
    if (compareFunction(array[low], array[low + 1]) > 0) {
      [array[low], array[low + 1]] = [array[low + 1], array[low]];
    }
    if (compareFunction(array[low + 2], array[high]) > 0) {
      [array[low + 2], array[high]] = [array[high], array[low + 2]];
    }
    if (compareFunction(array[low + 1], array[low + 2]) > 0) {
      [array[low + 1], array[low + 2]] = [array[low + 2], array[low + 1]];
    }
    return;
  }

  if (length === 5) {
    // 5-element sorting network
    if (compareFunction(array[low], array[low + 1]) > 0) {
      [array[low], array[low + 1]] = [array[low + 1], array[low]];
    }
    if (compareFunction(array[low + 3], array[high]) > 0) {
      [array[low + 3], array[high]] = [array[high], array[low + 3]];
    }
    if (compareFunction(array[low], array[low + 3]) > 0) {
      [array[low], array[low + 3]] = [array[low + 3], array[low]];
    }
    if (compareFunction(array[low + 1], array[high]) > 0) {
      [array[low + 1], array[high]] = [array[high], array[low + 1]];
    }
    if (compareFunction(array[low + 1], array[low + 2]) > 0) {
      [array[low + 1], array[low + 2]] = [array[low + 2], array[low + 1]];
    }
    if (compareFunction(array[low], array[low + 1]) > 0) {
      [array[low], array[low + 1]] = [array[low + 1], array[low]];
    }
    if (compareFunction(array[low + 2], array[low + 3]) > 0) {
      [array[low + 2], array[low + 3]] = [array[low + 3], array[low + 2]];
    }
    if (compareFunction(array[low + 1], array[low + 2]) > 0) {
      [array[low + 1], array[low + 2]] = [array[low + 2], array[low + 1]];
    }
    if (compareFunction(array[low + 3], array[high]) > 0) {
      [array[low + 3], array[high]] = [array[high], array[low + 3]];
    }
  }
};

/**
 * Find median of three elements
 */
export const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: CompareFunction<T>,
): number => {
  const ab = compareFunction(array[a], array[b]);
  if (ab < 0) {
    const bc = compareFunction(array[b], array[c]);
    if (bc < 0) {
      return b;
    }
    const ac = compareFunction(array[a], array[c]);
    return ac < 0 ? c : a;
  }
  const bc = compareFunction(array[b], array[c]);
  if (bc > 0) {
    return b;
  }
  const ac = compareFunction(array[a], array[c]);
  return ac > 0 ? c : a;
};
