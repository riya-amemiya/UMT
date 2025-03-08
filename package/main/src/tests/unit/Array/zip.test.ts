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

  it("should handle empty arrays correctly", () => {
    const array1 = [1, 2, 3];
    const emptyArray: number[] = [];
    const result = zip(array1, emptyArray);
    expect(result).toEqual([]);
  });

  it("should return an empty array when no arguments are provided", () => {
    const result = zip();
    expect(result).toEqual([]);
  });

  it("should handle arrays with null and undefined values", () => {
    const array1 = [1, null, 3];
    const array2 = ["a", undefined, "c"];
    const result = zip(array1, array2);
    expect(result).toEqual([
      [1, "a"],
      [null, undefined],
      [3, "c"],
    ]);
  });

  it("should handle nested arrays", () => {
    const array1 = [[1], [2], [3]];
    const array2 = [["a"], ["b"], ["c"]];
    const result = zip(array1, array2);
    expect(result).toEqual([
      [[1], ["a"]],
      [[2], ["b"]],
      [[3], ["c"]],
    ]);
  });

  it("should handle arrays of objects", () => {
    const array1 = [{ id: 1 }, { id: 2 }];
    const array2 = [{ name: "a" }, { name: "b" }];
    const result = zip(array1, array2);
    expect(result).toEqual([
      [{ id: 1 }, { name: "a" }],
      [{ id: 2 }, { name: "b" }],
    ]);
  });

  it("should handle mixed type arrays without type errors", () => {
    type Item = number | string | boolean;
    const numbers: Item[] = [1, 2];
    const strings: Item[] = ["a", "b"];
    const booleans: Item[] = [true, false];
    const result = zip(numbers, strings, booleans);
    expect(result).toEqual([
      [1, "a", true],
      [2, "b", false],
    ]);
  });
});
