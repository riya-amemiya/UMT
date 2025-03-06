import { mergeSort } from "@/Array/mergeSort";

describe("mergeSort", () => {
  it("should return an empty array when given an empty array", () => {
    expect(mergeSort([])).toEqual([]);
  });

  it("should return the same array when it's already sorted", () => {
    expect(mergeSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a reverse-sorted array", () => {
    expect(mergeSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a randomly ordered array", () => {
    expect(mergeSort([2, 3, 1])).toEqual([1, 2, 3]);
  });

  it("should correctly sort an array with duplicate elements", () => {
    expect(mergeSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });

  it("should handle a single-element array", () => {
    expect(mergeSort([1])).toEqual([1]);
  });

  it("should correctly sort an array with negative numbers", () => {
    expect(mergeSort([3, -1, 4, -5, 2, -3])).toEqual([-5, -3, -1, 2, 3, 4]);
  });

  it("should sort strings alphabetically", () => {
    expect(mergeSort(["banana", "apple", "cherry"])).toEqual([
      "apple",
      "banana",
      "cherry",
    ]);
  });

  it("should sort using custom comparison function", () => {
    // Sort in descending order
    const descendingCompare = (a: number, b: number) => b - a;
    expect(mergeSort([1, 3, 2], descendingCompare)).toEqual([3, 2, 1]);

    // Sort strings by length
    const lengthCompare = (a: string, b: string) => a.length - b.length;
    expect(mergeSort(["aaa", "a", "aa"], lengthCompare)).toEqual([
      "a",
      "aa",
      "aaa",
    ]);
  });

  it("should handle arrays with mixed types when using appropriate compare function", () => {
    const array = ["10", "2", "1"];
    const numericCompare = (a: string, b: string) => Number(a) - Number(b);
    expect(mergeSort(array, numericCompare)).toEqual(["1", "2", "10"]);
  });
});
