import { zip } from "@/Array/zip";

describe("zip function", () => {
  it("should combine multiple arrays", () => {
    const array1 = [1, 2, 3];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true];
    const result = zip(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [3, "three", true],
    ]);
  });

  it("should combine arrays of different lengths", () => {
    const array1 = [1, 2];
    const array2 = ["one", "two", "three"];
    const result = zip(array1, array2);
    expect(result).toEqual([
      [1, "one"],
      [2, "two"],
    ]);
  });

  it("should handle combination with empty arrays", () => {
    const array1 = [1, 2, 3];
    // @ts-ignore
    const array2 = [];
    // @ts-ignore
    const result = zip(array1, array2);
    expect(result).toEqual([]);
  });

  it("should return an empty array when no arguments are provided", () => {
    const result = zip();
    expect(result).toEqual([]);
  });
});
