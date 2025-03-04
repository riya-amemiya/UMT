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
});
