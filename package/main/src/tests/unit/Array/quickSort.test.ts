import { quickSort } from "@/Array/quickSort";
describe("quickSort", () => {
  it("should return an empty array when sorting an empty array", () => {
    expect(quickSort([])).toEqual([]);
  });

  it("should return the same array when it's already sorted", () => {
    expect(quickSort([1, 2, 3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
  });

  it("should correctly sort a reverse-sorted array", () => {
    expect(quickSort([5, 4, 3, 2, 1])).toEqual([1, 2, 3, 4, 5]);
  });

  it("should correctly sort a random array", () => {
    expect(quickSort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5])).toEqual([
      1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9,
    ]);
  });

  it("should correctly sort an array with duplicate elements", () => {
    expect(quickSort([3, 3, 3, 2, 1, 1, 4, 4, 5])).toEqual([
      1, 1, 2, 3, 3, 3, 4, 4, 5,
    ]);
  });

  it("should correctly sort an array with large number of elements", () => {
    const largeArray = Array.from({ length: 10000 }, () =>
      Math.floor(Math.random() * 10000),
    );
    const sortedArray = [...largeArray].sort((a, b) => a - b);
    expect(quickSort(largeArray)).toEqual(sortedArray);
  });

  it("should correctly sort an array with negative numbers", () => {
    expect(quickSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("should sort array in descending order", () => {
    expect(quickSort([1, 2, 3, 4, 5], (a, b) => b - a)).toEqual([
      5, 4, 3, 2, 1,
    ]);
  });

  it("should sort a portion of the array", () => {
    expect(quickSort([3, 1, 4, 1, 5], undefined, 1, 3)).toEqual([
      3, 1, 1, 4, 5,
    ]);
  });

  it("should return single-element array unchanged", () => {
    expect(quickSort([1])).toEqual([1]);
  });

  it("should handle compareFunction throwing an error", () => {
    const throwCompare = () => {
      throw new Error("Error during comparison");
    };
    expect(() => quickSort([3, 1, 4], throwCompare)).toThrow(
      "Error during comparison",
    );
  });

  describe("median of three cases", () => {
    it("should handle all elements equal", () => {
      expect(quickSort([5, 5, 5, 5, 5])).toEqual([5, 5, 5, 5, 5]);
    });

    it("should handle three-element arrays in all orders", () => {
      expect(quickSort([1, 2, 3])).toEqual([1, 2, 3]); // a < b < c
      expect(quickSort([1, 3, 2])).toEqual([1, 2, 3]); // a < c < b
      expect(quickSort([2, 1, 3])).toEqual([1, 2, 3]); // b < a < c
      expect(quickSort([2, 3, 1])).toEqual([1, 2, 3]); // b < c < a
      expect(quickSort([3, 1, 2])).toEqual([1, 2, 3]); // c < a < b
      expect(quickSort([3, 2, 1])).toEqual([1, 2, 3]); // c < b < a
    });
  });

  describe("insertion sort threshold cases", () => {
    it("should handle arrays at threshold size", () => {
      const arr = Array.from({ length: 10 }, (_, i) => 10 - i);
      expect(quickSort(arr)).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    });

    it("should handle arrays below threshold size", () => {
      const arr = Array.from({ length: 5 }, (_, i) => 5 - i);
      expect(quickSort(arr)).toEqual([1, 2, 3, 4, 5]);
    });

    it("should handle custom threshold", () => {
      const arr = Array.from({ length: 20 }, (_, i) => 20 - i);
      expect(quickSort(arr, undefined, 0, arr.length - 1, 5)).toEqual(
        Array.from({ length: 20 }, (_, i) => i + 1)
      );
    });
  });

  describe("boundary conditions", () => {
    it("should handle invalid index ranges", () => {
      // Out of bounds indices should be clamped to valid range
      expect(quickSort([3, 1, 4, 1, 5], undefined, -1, 10)).toEqual([1, 1, 3, 4, 5]);
      expect(quickSort([3, 1, 4, 1, 5], undefined, 10, -1)).toEqual([3, 1, 4, 1, 5]);

      // When startIndex > endIndex, array should remain unchanged
      const arr = [3, 1, 4, 1, 5];
      expect(quickSort([...arr], undefined, 3, 1)).toEqual(arr);

      // Sorting empty range should not change array
      expect(quickSort([...arr], undefined, 2, 2)).toEqual(arr);
    });

    it("should handle two-element arrays", () => {
      expect(quickSort([2, 1])).toEqual([1, 2]);
      expect(quickSort([1, 2])).toEqual([1, 2]);
    });

    it("should handle arrays with all same value except one", () => {
      expect(quickSort([1, 1, 1, 0, 1])).toEqual([0, 1, 1, 1, 1]);
      expect(quickSort([1, 1, 1, 2, 1])).toEqual([1, 1, 1, 1, 2]);
    });
  });
});
