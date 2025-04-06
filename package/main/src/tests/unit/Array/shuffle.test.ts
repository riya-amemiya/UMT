import { shuffle } from "@/Array/shuffle";

describe("shuffle function", () => {
  it("should shuffle array elements", () => {
    const array = [1, 2, 3, 4, 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
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
    const array = [1, "2", 3, "4", 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
  });

  it("should shuffle large arrays", () => {
    const array = Array.from({ length: 1000 }, (_, index) => index);
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
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
    const array = [
      { id: 1, value: "one" },
      { id: 2, value: "two" },
      { id: 3, value: "three" },
      { id: 4, value: "four" },
      { id: 5, value: "five" },
    ];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.map((obj) => obj.id).sort()).toEqual(
      array.map((obj) => obj.id).sort(),
    );
  });

  it("should not modify the original array", () => {
    const array = [1, 2, 3, 4, 5];
    const originalArray = [...array];
    shuffle(array);
    expect(array).toEqual(originalArray);
  });
});
