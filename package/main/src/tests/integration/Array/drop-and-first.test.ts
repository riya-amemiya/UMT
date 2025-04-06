import { first } from "@/Array/first";
import { drop } from "@/Array/drop";

/**
 * Integration tests for Array utility functions
 *
 * Tests the interaction between `drop` and `first` functions to ensure they work correctly
 * when used together in various scenarios:
 * - Dropping elements from the start (left) and getting the first element
 * - Dropping elements from the end (right) and getting the first element
 * - Edge cases with different array sizes and drop amounts
 */
describe("Integration test for 'drop' and 'first' functions", () => {
  it("should drop n elements from the array and return the first element", () => {
    expect(first(drop([1, 2, 3, 4, 5]))).toBe(2);
    expect(first(drop([1, 2, 3, 4, 5], 2))).toBe(3);
    expect(first(drop([1, 2, 3, 4, 5], 3))).toBe(4);
    expect(first(drop([1, 2, 3, 4, 5], 4))).toBe(5);
    expect(first(drop([1, 2, 3, 4, 5], 2, "right"))).toBe(1);
    expect(first(drop([1, 2, 3, 4, 5], 3, "left"))).toBe(4);
  });

  it("should handle edge cases with empty arrays and large drop amounts", () => {
    expect(first(drop([]))).toBeUndefined();
    expect(first(drop([1], 1))).toBeUndefined();
    expect(first(drop([1, 2], 5))).toBeUndefined();
    expect(first(drop([1, 2, 3], 0))).toBe(1);
  });

  it("should handle arrays with different data types", () => {
    expect(first(drop(["a", "b", "c"], 1))).toBe("b");
    expect(first(drop([true, false], 1))).toBe(false);
    expect(first(drop([null, undefined, 1], 2))).toBe(1);
  });
});
