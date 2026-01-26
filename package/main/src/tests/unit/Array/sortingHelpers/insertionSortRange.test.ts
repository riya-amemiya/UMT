import { insertionSortRange } from "@/Array/sortingHelpers/insertionSortRange";

describe("insertionSortRange function", () => {
  const numberCompare = (a: number, b: number) => a - b;
  const stringCompare = (a: string, b: string) => a.localeCompare(b);

  describe("basic sorting", () => {
    test("should sort entire array when range covers all elements", () => {
      const array = [3, 1, 4, 1, 5, 9, 2, 6];
      insertionSortRange(array, numberCompare, 0, 7);
      expect(array).toEqual([1, 1, 2, 3, 4, 5, 6, 9]);
    });

    test("should sort array in ascending order", () => {
      const array = [5, 2, 8, 1, 9];
      insertionSortRange(array, numberCompare, 0, 4);
      expect(array).toEqual([1, 2, 5, 8, 9]);
    });

    test("should handle already sorted array", () => {
      const array = [1, 2, 3, 4, 5];
      insertionSortRange(array, numberCompare, 0, 4);
      expect(array).toEqual([1, 2, 3, 4, 5]);
    });

    test("should handle reverse sorted array", () => {
      const array = [5, 4, 3, 2, 1];
      insertionSortRange(array, numberCompare, 0, 4);
      expect(array).toEqual([1, 2, 3, 4, 5]);
    });
  });

  describe("partial range sorting", () => {
    test("should sort only the specified range", () => {
      const array = [9, 5, 2, 8, 3, 7, 1];
      insertionSortRange(array, numberCompare, 2, 5);
      expect(array).toEqual([9, 5, 2, 3, 7, 8, 1]);
    });

    test("should not modify elements before start index", () => {
      const array = [9, 8, 3, 1, 4, 2];
      insertionSortRange(array, numberCompare, 2, 5);
      expect(array[0]).toBe(9);
      expect(array[1]).toBe(8);
    });

    test("should not modify elements after end index", () => {
      const array = [5, 3, 1, 4, 2, 9, 8];
      insertionSortRange(array, numberCompare, 0, 4);
      expect(array[5]).toBe(9);
      expect(array[6]).toBe(8);
    });
  });

  describe("edge cases", () => {
    test("should handle single element range", () => {
      const array = [3, 1, 4];
      insertionSortRange(array, numberCompare, 1, 1);
      expect(array).toEqual([3, 1, 4]);
    });

    test("should handle two element range", () => {
      const array = [3, 5, 2, 4];
      insertionSortRange(array, numberCompare, 1, 2);
      expect(array).toEqual([3, 2, 5, 4]);
    });

    test("should handle array with duplicate values", () => {
      const array = [3, 1, 3, 1, 2];
      insertionSortRange(array, numberCompare, 0, 4);
      expect(array).toEqual([1, 1, 2, 3, 3]);
    });
  });

  describe("different data types", () => {
    test("should work with string arrays", () => {
      const array = ["banana", "apple", "cherry", "date"];
      insertionSortRange(array, stringCompare, 0, 3);
      expect(array).toEqual(["apple", "banana", "cherry", "date"]);
    });

    test("should work with object arrays", () => {
      const array = [
        { id: 3, name: "Charlie" },
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
      ];
      const objectCompare = (a: (typeof array)[0], b: (typeof array)[0]) =>
        a.id - b.id;

      insertionSortRange(array, objectCompare, 0, 2);
      expect(array).toEqual([
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
        { id: 3, name: "Charlie" },
      ]);
    });
  });

  describe("custom compare functions", () => {
    test("should respect descending order compare function", () => {
      const array = [1, 3, 2, 4];
      const descendingCompare = (a: number, b: number) => b - a;
      insertionSortRange(array, descendingCompare, 0, 3);
      expect(array).toEqual([4, 3, 2, 1]);
    });
  });
});
