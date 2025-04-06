import { dualPivotQuickSort } from "../../../Array/dualPivotQuickSort";

describe("dualPivotQuickSort", () => {
  test("should sort an array of numbers in ascending order", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    expect(dualPivotQuickSort([...array])).toEqual([
      1, 1, 2, 3, 3, 4, 5, 5, 6, 9,
    ]);
  });

  test("should sort an array of strings lexicographically", () => {
    const array = ["banana", "apple", "orange", "grape"];
    expect(dualPivotQuickSort([...array])).toEqual([
      "apple",
      "banana",
      "grape",
      "orange",
    ]);
  });

  test("should sort in descending order when custom comparison function is provided", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    const compareFunction = (a: number, b: number) => b - a;
    expect(dualPivotQuickSort([...array], compareFunction)).toEqual([
      9, 6, 5, 5, 4, 3, 3, 2, 1, 1,
    ]);
  });

  test("should handle empty array", () => {
    expect(dualPivotQuickSort([])).toEqual([]);
  });

  test("should handle array with single element", () => {
    expect(dualPivotQuickSort([1])).toEqual([1]);
  });

  test("should handle array with all identical elements", () => {
    const array = [2, 2, 2, 2, 2];
    expect(dualPivotQuickSort([...array])).toEqual([2, 2, 2, 2, 2]);
  });

  test("should handle already sorted array", () => {
    const array = [1, 2, 3, 4, 5];
    expect(dualPivotQuickSort([...array])).toEqual([1, 2, 3, 4, 5]);
  });

  test("should handle reverse sorted array", () => {
    const array = [5, 4, 3, 2, 1];
    expect(dualPivotQuickSort([...array])).toEqual([1, 2, 3, 4, 5]);
  });

  test("should handle array with negative numbers", () => {
    const array = [-3, 1, -4, 1, 5, -9, 2, 6, 5, 3];
    expect(dualPivotQuickSort([...array])).toEqual([
      -9, -4, -3, 1, 1, 2, 3, 5, 5, 6,
    ]);
  });

  test("should sort only specified range of array", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    expect(dualPivotQuickSort([...array], undefined, 2, 6)).toEqual([
      3, 1, 1, 2, 4, 5, 9, 6, 5, 3,
    ]);
  });

  test("should handle large arrays efficiently", () => {
    const size = 1000;
    const array = Array.from({ length: size }, () =>
      Math.floor(Math.random() * size),
    );
    const sorted = dualPivotQuickSort([...array]);
    for (let i = 1; i < sorted.length; i++) {
      expect(sorted[i]).toBeGreaterThanOrEqual(sorted[i - 1]);
    }
  });

  test("should use insertion sort for small arrays when threshold is adjusted", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    expect(
      dualPivotQuickSort([...array], undefined, 0, array.length - 1, 5),
    ).toEqual([1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
  });

  test("should sort array of objects using custom comparison", () => {
    const array = [
      { id: 3, value: "c" },
      { id: 1, value: "a" },
      { id: 2, value: "b" },
    ];
    const compareFunction = (a: { id: number }, b: { id: number }) =>
      a.id - b.id;
    expect(dualPivotQuickSort([...array], compareFunction)).toEqual([
      { id: 1, value: "a" },
      { id: 2, value: "b" },
      { id: 3, value: "c" },
    ]);
  });
});
