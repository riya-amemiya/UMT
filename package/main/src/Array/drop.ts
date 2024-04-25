/**
 * 配列の先頭からn個の要素を除外した新しい配列を返します。
 * @param {T[]} array 対象の配列
 * @param {number} n 除外する要素の数
 * @param {"left" | "right" | "between"} direction 除外する方向
 * @returns n個の要素を除外した新しい配列
 *
 * @example drop([1, 2, 3, 4, 5], 2); // [3, 4, 5]
 * @example drop([1, 2, 3, 4, 5], 2, "left"); // [3, 4, 5]
 * @example drop([1, 2, 3, 4, 5], 2, "right"); // [1, 2, 3]
 * @example drop([1, 2, 3, 4, 5], 1, "between"); // [1, 2, 4, 5]
 */
export const drop = <T>(
  array: T[],
  n = 1,
  direction: "left" | "right" | "between" = "left",
): T[] => {
  if (n < 0) {
    return array;
  }

  switch (direction) {
    case "left": {
      return array.slice(n);
    }
    case "right": {
      return array.slice(0, array.length - n);
    }
    case "between": {
      const mid = Math.floor(array.length / 2);
      const start = mid - Math.floor(n / 2);
      const end = mid + Math.ceil(n / 2);
      return [...array.slice(0, start), ...array.slice(end)];
    }
    default: {
      return array.slice(n);
    }
  }
};
