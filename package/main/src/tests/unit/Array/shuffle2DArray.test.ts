import { shuffle2DArray } from "@/Array/shuffle2DArray";

describe("shuffle2DArray function", () => {
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
    shuffledArray.forEach((subArray, index) => {
      expect(subArray.length).toBe(array[index].length);
    });
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
});
