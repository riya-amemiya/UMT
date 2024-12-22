/**
 * 二分探索法
 * @param arr sortされたnumberの配列
 * @param target 探索する値
 */
export const binarySearch = (array: number[], target: number): number => {
  let left = 0;
  let right = array.length - 1;

  while (left <= right) {
    const mid = Math.floor((left + right) / 2);
    const midValue = array[mid];

    if (midValue === target) {
      return mid;
    }
    if (midValue < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  return -1;
};
