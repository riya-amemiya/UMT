import { timSort } from "@/Array/timSort";

describe("timSort", () => {
  it("should return an empty array when sorting an empty array", () => {
    expect(timSort([])).toEqual([]);
  });

  it("should return the same array when it's already sorted", () => {
    expect(timSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a reverse-sorted array", () => {
    expect(timSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a random array", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    const sortedArray = [...array].sort((a, b) => a - b);
    expect(timSort(array)).toEqual(sortedArray);
  });

  it("should correctly sort an array with duplicate elements", () => {
    expect(timSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });

  it("should correctly sort an array with large number of elements", () => {
    const largeArray = Array.from({ length: 10000 }, () =>
      Math.floor(Math.random() * 10000),
    );
    const sortedArray = [...largeArray].sort((a, b) => a - b);
    expect(timSort(largeArray)).toEqual(sortedArray);
  });

  it("should correctly sort an array with negative numbers", () => {
    expect(timSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("should sort array in descending order", () => {
    expect(timSort([1, 2, 3], (a, b) => b - a)).toEqual([3, 2, 1]);
  });

  it("should sort a portion of the array", () => {
    expect(timSort([1, 3, 2, 5, 4], (a, b) => a - b, 1, 3)).toEqual([
      1, 2, 3, 5, 4,
    ]);
  });
});
