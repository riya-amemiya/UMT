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
});
