import { addition } from "@/Math/addition";
/**
 * Returns the sum of an array of numbers
 * @param {number[]} x Array of numbers
 * @returns Sum of the array elements
 * @example sum([1, 2, 3]); // 6
 */
export const sum = (x: number[]) => {
  return x.reduce((a, b) => addition(a, b), 0);
};
