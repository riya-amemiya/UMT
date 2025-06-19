import { shuffle2DArray } from "@/Array/shuffle2DArray";

describe("shuffle2DArray function", () => {
  it("should handle single-element 2D array", () => {
    const array = [[1]];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray).toEqual(array);
  });

  it("should return empty 2D array unchanged", () => {
    const array: number[][] = [];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray).toEqual(array);
  });

  it("should return array with empty subarrays unchanged", () => {
    const array = [[], [], []];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray).toEqual(array);
  });

  it("should maintain the same length of 2D array and subarrays after shuffling", () => {
    const array = [
      [1, 2],
      [3, 4],
      [5, 6],
    ];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray.length).toBe(array.length);
    for (const [index, subArray] of shuffledArray.entries()) {
      expect(subArray.length).toBe(array[index].length);
    }
  });

  it("should shuffle 2D array with mixed types", () => {
    const array = [
      [1, "2"],
      ["3", 4],
      [5, "6"],
    ];
    const shuffledArray = shuffle2DArray(array);
    // Check if the entire array is shuffled
    expect(shuffledArray.map((subArray) => subArray.sort())).not.toEqual(
      array.map((subArray) => subArray.sort()),
    );
  });

  it("should shuffle large 2D arrays", () => {
    const array = Array.from({ length: 1000 }, (_, index) => [
      index,
      index + 1,
    ]);
    const shuffledArray = shuffle2DArray(array);
    // Check if the entire array is shuffled
    expect(shuffledArray.map((subArray) => subArray.sort())).not.toEqual(
      array.map((subArray) => subArray.sort()),
    );
  });

  it("should handle subarrays of different lengths", () => {
    const array = [[1], [2, 3], [4, 5, 6]];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray.length).toBe(array.length);
    for (const [index, subArray] of shuffledArray.entries()) {
      expect(subArray.length).toBe(array[index].length);
    }
    // Verify the total number of elements remains the same
    const flatOriginal = array.flat();
    const flatShuffled = shuffledArray.flat();
    expect(flatShuffled.sort()).toEqual(flatOriginal.sort());
  });

  it("should not modify the original array", () => {
    const array = [
      [1, 2],
      [3, 4],
      [5, 6],
    ];
    const originalArray = array.map((subArray) => [...subArray]);
    shuffle2DArray(array);
    expect(array).toEqual(originalArray);
  });

  it("should maintain the total count of each unique element", () => {
    const array = [
      [1, 1],
      [2, 2],
      [1, 2],
    ];
    const shuffledArray = shuffle2DArray(array);
    const countElements = (arr: number[][]) => {
      const counts: Record<number, number> = {};
      for (const num of arr.flat()) {
        counts[num] = (counts[num] || 0) + 1;
      }
      return counts;
    };
    expect(countElements(shuffledArray)).toEqual(countElements(array));
  });
});
