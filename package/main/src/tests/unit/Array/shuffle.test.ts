import { shuffle } from "@/Array/shuffle";

describe("shuffle function", () => {
  it("should shuffle array elements", () => {
    // Increase array size to reduce probability of shuffle returning original order
    const array = Array.from({ length: 20 }, (_, i) => i + 1);
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort((a, b) => a - b)).toEqual(
      array.sort((a, b) => a - b),
    );
  });

  it("should return empty array unchanged", () => {
    const array: number[] = [];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).toEqual(array);
  });

  it("should maintain the same length after shuffling", () => {
    const array = [1, 2, 3, 4, 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray.length).toBe(array.length);
  });

  it("should shuffle array with mixed strings and numbers", () => {
    const array = [
      1,
      "2",
      3,
      "4",
      5,
      6,
      "7",
      8,
      "9",
      10,
      11,
      12,
      13,
      14,
      15,
      16,
      17,
      18,
      19,
      20,
    ];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    // Sort logic needs to be consistent for mixed types
    const sortFn = (a: string | number, b: string | number) =>
      String(a).localeCompare(String(b));
    expect(shuffledArray.sort(sortFn)).toEqual(array.sort(sortFn));
  });

  it("should shuffle large arrays", () => {
    const array = Array.from({ length: 1000 }, (_, index) => index);
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort((a, b) => a - b)).toEqual(
      array.sort((a, b) => a - b),
    );
  });

  it("should return single-element array unchanged", () => {
    const array = [1];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).toEqual(array);
  });

  it("should maintain frequency of duplicate elements", () => {
    const array = [1, 1, 2, 2, 3, 3];
    const shuffledArray = shuffle(array);
    const countElements = (arr: number[]) =>
      arr.reduce(
        (acc, curr) => {
          acc[curr] = (acc[curr] || 0) + 1;
          return acc;
        },
        {} as Record<number, number>,
      );

    expect(countElements(shuffledArray)).toEqual(countElements(array));
  });

  it("should handle array of objects", () => {
    const array = Array.from({ length: 20 }, (_, i) => ({
      id: i,
      value: `val${i}`,
    }));
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.map((obj) => obj.id).sort((a, b) => a - b)).toEqual(
      array.map((obj) => obj.id).sort((a, b) => a - b),
    );
  });

  it("should not modify the original array", () => {
    const array = [1, 2, 3, 4, 5];
    const originalArray = [...array];
    shuffle(array);
    expect(array).toEqual(originalArray);
  });
});
