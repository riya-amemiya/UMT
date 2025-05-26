import { ultraSort } from "@/Array/ultraSort";

describe("ultraSort", () => {
  it("should return an empty array when given an empty array", () => {
    expect(ultraSort([])).toEqual([]);
  });

  it("should handle a single-element array", () => {
    expect(ultraSort([42])).toEqual([42]);
  });

  it("should sort very small arrays (≤5 elements) using network sort", () => {
    expect(ultraSort([3, 1, 4, 1, 5])).toEqual([1, 1, 3, 4, 5]);
    expect(ultraSort([5, 4, 3, 2, 1])).toEqual([1, 2, 3, 4, 5]);
    expect(ultraSort([2, 1])).toEqual([1, 2]);
    expect(ultraSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should sort small arrays (≤24 elements) using insertion sort", () => {
    const array = [15, 3, 9, 1, 5, 8, 2, 7, 4, 6, 10, 12, 11, 14, 13];
    const expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should sort large arrays using introsort", () => {
    const array = Array.from({ length: 100 }, (_, i) => 100 - i);
    const expected = Array.from({ length: 100 }, (_, i) => i + 1);
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should handle arrays with duplicate elements", () => {
    const array = [5, 2, 8, 2, 9, 1, 5, 5, 2, 8];
    const expected = [1, 2, 2, 2, 5, 5, 5, 8, 8, 9];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should handle already sorted arrays", () => {
    const array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    expect(ultraSort(array)).toEqual(array);
  });

  it("should handle reverse sorted arrays", () => {
    const array = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    const expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should handle arrays with negative numbers", () => {
    const array = [3, -1, 4, -5, 2, -3, 0];
    const expected = [-5, -3, -1, 0, 2, 3, 4];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should handle arrays with floating point numbers", () => {
    const array = [3.14, 2.71, 1.41, 1.73, 0.57];
    const expected = [0.57, 1.41, 1.73, 2.71, 3.14];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should sort strings alphabetically", () => {
    const array = ["zebra", "apple", "banana", "cherry", "date"];
    const expected = ["apple", "banana", "cherry", "date", "zebra"];
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should work with custom comparison function for descending order", () => {
    const array = [1, 5, 3, 9, 2, 8, 4, 7, 6];
    const descendingCompare = (a: number, b: number) => b - a;
    const expected = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    expect(ultraSort(array, descendingCompare)).toEqual(expected);
  });

  it("should work with custom comparison function for string length", () => {
    const array = ["hello", "a", "world", "test", "ab"];
    const lengthCompare = (a: string, b: string) => a.length - b.length;
    const expected = ["a", "ab", "test", "world", "hello"];
    expect(ultraSort(array, lengthCompare)).toEqual(expected);
  });

  it("should handle large arrays with many duplicates", () => {
    const array = Array.from({ length: 200 }, (_, i) => i % 10);
    const expected = Array.from({ length: 200 }, (_, i) => Math.floor(i / 20));
    expect(ultraSort(array)).toEqual(expected);
  });

  it("should handle arrays with objects using custom comparison", () => {
    const array = [
      { id: 3, name: "Charlie" },
      { id: 1, name: "Alice" },
      { id: 2, name: "Bob" },
    ];
    const idCompare = (a: { id: number }, b: { id: number }) => a.id - b.id;
    const expected = [
      { id: 1, name: "Alice" },
      { id: 2, name: "Bob" },
      { id: 3, name: "Charlie" },
    ];
    expect(ultraSort(array, idCompare)).toEqual(expected);
  });

  it("should maintain stability for equal elements with custom comparison", () => {
    const array = [
      { value: 1, order: "first" },
      { value: 2, order: "first" },
      { value: 1, order: "second" },
      { value: 2, order: "second" },
    ];
    const valueCompare = (a: { value: number }, b: { value: number }) =>
      a.value - b.value;
    const result = ultraSort(array, valueCompare);

    expect(result[0].value).toBe(1);
    expect(result[1].value).toBe(1);
    expect(result[2].value).toBe(2);
    expect(result[3].value).toBe(2);
  });

  it("should handle very large arrays efficiently", () => {
    const size = 1000;
    const array = Array.from({ length: size }, () =>
      Math.floor(Math.random() * size),
    );
    const result = ultraSort([...array]);

    for (let i = 1; i < result.length; i++) {
      expect(result[i]).toBeGreaterThanOrEqual(result[i - 1]);
    }
    expect(result.length).toBe(size);
  });

  it("should sort the array in-place and return the same reference", () => {
    const original = [3, 1, 4, 1, 5];
    const result = ultraSort(original);
    expect(result).toBe(original);
    expect(result).toEqual([1, 1, 3, 4, 5]);
    expect(original).toEqual([1, 1, 3, 4, 5]);
  });

  it("should handle large arrays efficiently", () => {
    const size = 10000;
    const array = Array.from({ length: size }, (_, i) => size - i);

    const result = ultraSort(array);

    for (let i = 1; i < result.length; i++) {
      expect(result[i]).toBeGreaterThanOrEqual(result[i - 1]);
    }

    expect(result[0]).toBe(1);
    expect(result[result.length - 1]).toBe(size);
  });
});
