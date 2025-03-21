import { insertionSort } from "@/Array/insertionSort";

describe("insertionSort", () => {
  it("should return an empty array when sorting an empty array", () => {
    expect(insertionSort([])).toEqual([]);
  });

  it("should return the same array when it's already sorted", () => {
    expect(insertionSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a reverse-sorted array", () => {
    expect(insertionSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a random array", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    const sortedArray = [...array].sort((a, b) => a - b);
    expect(insertionSort(array)).toEqual(sortedArray);
  });

  it("should correctly sort an array with duplicate elements", () => {
    expect(insertionSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });

  it("should correctly sort an array with negative numbers", () => {
    expect(insertionSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("should sort array in descending order", () => {
    expect(insertionSort([1, 2, 3], (a, b) => b - a)).toEqual([3, 2, 1]);
  });

  it("should sort a portion of the array", () => {
    expect(insertionSort([1, 3, 2, 5, 4], (a, b) => a - b, 1, 3)).toEqual([
      1, 2, 3, 5, 4,
    ]);
  });
});
