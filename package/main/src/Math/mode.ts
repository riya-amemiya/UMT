import { ultraNumberSort } from "@/Array/ultraNumberSort";

/**
 * Finds the most frequently occurring value(s) in an array
 * @param array - Array of numbers to find mode for
 * @returns Array of mode values (can be multiple values if there are ties)
 * @example
 * mode([1, 2, 2, 3, 3, 3]); // [3]
 * mode([1, 2, 2, 3, 3]); // [2, 3]
 * mode([1, 2, 3]); // [1, 2, 3]
 */
export const mode = (array: number[]): number[] => {
  if (array.length === 0) {
    return [];
  }

  const frequency = new Map<number, number>();
  let maxFrequency = 0;

  // Count frequencies
  for (const value of array) {
    const count = (frequency.get(value) || 0) + 1;
    frequency.set(value, count);
    maxFrequency = Math.max(maxFrequency, count);
  }

  // Find all values with maximum frequency
  const modes: number[] = [];
  for (const [value, count] of frequency) {
    if (count === maxFrequency) {
      modes.push(value);
    }
  }

  return ultraNumberSort(modes);
};
