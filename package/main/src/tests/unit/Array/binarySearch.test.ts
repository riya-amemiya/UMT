import { binarySearch } from "@/Array/binarySearch";

describe("binarySearch", () => {
  it("should return the correct index when the target is present", () => {
    const array = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    expect(binarySearch(array, 7)).toBe(3);
    expect(binarySearch(array, 1)).toBe(0);
    expect(binarySearch(array, 19)).toBe(9);
  });

  it("should return -1 when the target is not present", () => {
    const array = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    expect(binarySearch(array, 2)).toBe(-1);
    expect(binarySearch(array, 20)).toBe(-1);
    expect(binarySearch(array, 0)).toBe(-1);
  });

  it("should handle an empty array", () => {
    const array: number[] = [];
    expect(binarySearch(array, 1)).toBe(-1);
  });

  it("should handle a single-element array", () => {
    const array = [5];
    expect(binarySearch(array, 5)).toBe(0);
    expect(binarySearch(array, 1)).toBe(-1);
  });

  it("should handle a two-element array", () => {
    const array = [3, 6];
    expect(binarySearch(array, 3)).toBe(0);
    expect(binarySearch(array, 6)).toBe(1);
    expect(binarySearch(array, 4)).toBe(-1);
  });
});
