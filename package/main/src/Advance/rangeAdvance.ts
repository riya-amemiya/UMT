import { range } from "@/Array/range";

/**
 * Returns an array of numbers that satisfy the conditional expression
 * @param start - Starting number
 * @param end - Ending number (exclusive)
 * @param conditionalExpression - Function that determines which numbers to include
 * @returns Array of numbers that satisfy the conditional expression
 *
 * @example
 * ```ts
 * rangeAdvance(1, 10, (number) => number % 2 === 0); // [2, 4, 6, 8]
 * ```
 */
const rangeAdvance = (
  start: number,
  end?: number,
  conditionalExpression?: (number_: number) => boolean,
): number[] => {
  if (conditionalExpression) {
    const array: number[] = [];
    for (let index = start; index < (end as number); index++) {
      if (conditionalExpression(index)) {
        array.push(index);
      }
    }
    return array;
  }
  return range(start, end);
};
export { rangeAdvance };
