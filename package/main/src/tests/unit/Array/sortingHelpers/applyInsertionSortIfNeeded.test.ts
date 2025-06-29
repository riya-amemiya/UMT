import { applyInsertionSortIfNeeded } from "@/Array/sortingHelpers/applyInsertionSortIfNeeded";

describe("applyInsertionSortIfNeeded function", () => {
  const numberCompare = (a: number, b: number) => a - b;
  const stringCompare = (a: string, b: string) => a.localeCompare(b);

  describe("threshold-based decision making", () => {
    test("should apply insertion sort when partition size equals threshold", () => {
      const array = [3, 1, 4, 1, 5];
      const threshold = 5;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        4,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 1, 3, 4, 5]);
    });

    test("should apply insertion sort when partition size is below threshold", () => {
      const array = [5, 2, 8, 1, 9];
      const threshold = 10;

      const result = applyInsertionSortIfNeeded(
        array,
        1,
        3,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([5, 1, 2, 8, 9]);
    });

    test("should not apply insertion sort when partition size exceeds threshold", () => {
      const array = [5, 2, 8, 1, 9, 3, 7, 6];
      const originalArray = [...array];
      const threshold = 3;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        7,
        numberCompare,
        threshold,
      );

      expect(result).toBe(false);
      expect(array).toEqual(originalArray); // Should remain unchanged
    });
  });

  describe("partial array sorting", () => {
    test("should sort only the specified range", () => {
      const array = [1, 5, 2, 8, 3, 7, 4];
      const threshold = 5;

      const result = applyInsertionSortIfNeeded(
        array,
        2,
        5,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 5, 2, 3, 7, 8, 4]); // Only indices 2-5 sorted
    });

    test("should handle single element partition", () => {
      const array = [3, 1, 4, 1, 5];
      const threshold = 1;

      const result = applyInsertionSortIfNeeded(
        array,
        2,
        2,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([3, 1, 4, 1, 5]); // Single element, no change needed
    });

    test("should handle two element partition", () => {
      const array = [1, 4, 2, 3, 5];
      const threshold = 2;

      const result = applyInsertionSortIfNeeded(
        array,
        1,
        2,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 2, 4, 3, 5]); // Swap elements at indices 1 and 2
    });
  });

  describe("different data types", () => {
    test("should work with string arrays", () => {
      const array = ["banana", "apple", "cherry", "date"];
      const threshold = 4;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        3,
        stringCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual(["apple", "banana", "cherry", "date"]);
    });

    test("should work with object arrays", () => {
      const array = [
        { id: 3, name: "Charlie" },
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
      ];
      const threshold = 3;
      const objectCompare = (a: (typeof array)[0], b: (typeof array)[0]) =>
        a.id - b.id;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        2,
        objectCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
        { id: 3, name: "Charlie" },
      ]);
    });
  });

  describe("edge cases", () => {
    test("should handle empty partition", () => {
      const array = [1, 2, 3];
      const threshold = 1;

      // This creates an invalid range, but let's see how it behaves
      const result = applyInsertionSortIfNeeded(
        array,
        1,
        0,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true); // Size calculation: 0 - 1 + 1 = 0, which is <= threshold
    });

    test("should handle zero threshold", () => {
      const array = [3, 1, 2];
      const threshold = 0;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        2,
        numberCompare,
        threshold,
      );

      expect(result).toBe(false); // Size 3 > threshold 0
    });

    test("should handle large threshold", () => {
      const array = [3, 1, 4, 1, 5, 9, 2, 6];
      const threshold = 1000;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        7,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 1, 2, 3, 4, 5, 6, 9]);
    });
  });

  describe("threshold boundary conditions", () => {
    test("should apply sort when size exactly equals threshold", () => {
      const array = [4, 2, 3, 1];
      const threshold = 4; // Exactly the size of the array

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        3,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 2, 3, 4]);
    });

    test("should not apply sort when size is one more than threshold", () => {
      const array = [4, 2, 3, 1, 5];
      const originalArray = [...array];
      const threshold = 4; // One less than array size

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        4,
        numberCompare,
        threshold,
      );

      expect(result).toBe(false);
      expect(array).toEqual(originalArray);
    });
  });

  describe("compare function behavior", () => {
    test("should respect custom compare function for descending order", () => {
      const array = [1, 3, 2, 4];
      const descendingCompare = (a: number, b: number) => b - a;
      const threshold = 5;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        3,
        descendingCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([4, 3, 2, 1]);
    });

    test("should handle complex compare functions", () => {
      const array = [
        { value: 10, priority: 2 },
        { value: 20, priority: 1 },
        { value: 15, priority: 3 },
      ];
      const threshold = 5;
      // Sort by priority first, then by value
      const complexCompare = (a: (typeof array)[0], b: (typeof array)[0]) => {
        if (a.priority !== b.priority) {
          return a.priority - b.priority;
        }
        return a.value - b.value;
      };

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        2,
        complexCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([
        { value: 20, priority: 1 },
        { value: 10, priority: 2 },
        { value: 15, priority: 3 },
      ]);
    });
  });

  describe("performance characteristics", () => {
    test("should efficiently handle typical small partitions", () => {
      const array = Array.from({ length: 10 }, (_, i) => 10 - i); // [10, 9, 8, ..., 1]
      const threshold = 10;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        9,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    });

    test("should handle already sorted arrays efficiently", () => {
      const array = [1, 2, 3, 4, 5];
      const threshold = 5;

      const result = applyInsertionSortIfNeeded(
        array,
        0,
        4,
        numberCompare,
        threshold,
      );

      expect(result).toBe(true);
      expect(array).toEqual([1, 2, 3, 4, 5]); // Should remain the same
    });
  });
});
