/**
 * Returns a new array with n elements removed from the specified direction
 * @param {T[]} array The target array
 * @param {number} n The number of elements to remove
 * @param {"left" | "right" | "between"} direction The direction to remove elements from
 * @returns A new array with n elements removed
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
