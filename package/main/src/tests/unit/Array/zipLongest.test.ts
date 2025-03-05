import { zipLongest } from "@/Array/zipLongest";

describe("zipLongest function", () => {
  it("should combine arrays of different lengths with undefined padding", () => {
    const array1 = [1, 2];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true, false];
    const result = zipLongest(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [undefined, "three", true],
      [undefined, undefined, false],
    ]);
  });

  it("should combine arrays of equal length", () => {
    const array1 = [1, 2, 3];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true];
    const result = zipLongest(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [3, "three", true],
    ]);
  });

  it("should handle combination with empty arrays using undefined padding", () => {
    const array1 = [1, 2, 3];
    // @ts-ignore
    const array2 = [];
    // @ts-ignore
    const result = zipLongest(array1, array2);
    expect(result).toEqual([
      [1, undefined],
      [2, undefined],
      [3, undefined],
    ]);
  });

  it("should return an empty array when no arguments are provided", () => {
    const result = zipLongest();
    expect(result).toEqual([]);
  });
});
