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

  it("should handle empty arrays correctly", () => {
    const array1 = [1, 2, 3];
    const emptyArray: number[] = [];
    const result = zipLongest(array1, emptyArray);
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

  it("should handle arrays with null and undefined values", () => {
    const array1 = [1, null, 3];
    const array2 = ["a", undefined, "c", "d"];
    const result = zipLongest(array1, array2);
    expect(result).toEqual([
      [1, "a"],
      [null, undefined],
      [3, "c"],
      [undefined, "d"],
    ]);
  });

  it("should handle nested arrays", () => {
    const array1 = [[1], [2]];
    const array2 = [["a"], ["b"], ["c"]];
    const result = zipLongest(array1, array2);
    expect(result).toEqual([
      [[1], ["a"]],
      [[2], ["b"]],
      [undefined, ["c"]],
    ]);
  });

  it("should handle arrays of objects", () => {
    const array1 = [{ id: 1 }, { id: 2 }];
    const array2 = [{ name: "a" }, { name: "b" }, { name: "c" }];
    const result = zipLongest(array1, array2);
    expect(result).toEqual([
      [{ id: 1 }, { name: "a" }],
      [{ id: 2 }, { name: "b" }],
      [undefined, { name: "c" }],
    ]);
  });

  it("should handle mixed type arrays with type safety", () => {
    type Item = number | string | boolean;
    const numbers: Item[] = [1, 2];
    const strings: Item[] = ["a", "b", "c"];
    const booleans: Item[] = [true];
    const result = zipLongest(numbers, strings, booleans);
    expect(result).toEqual([
      [1, "a", true],
      [2, "b", undefined],
      [undefined, "c", undefined],
    ]);
  });
});
