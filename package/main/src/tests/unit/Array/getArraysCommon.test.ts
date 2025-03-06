import { getArraysCommon } from "@/Array/getArraysCommon";

describe("getArraysCommon", () => {
  it("should find common elements between arrays", () => {
    // Test with two arrays
    expect(getArraysCommon([1, 2, 3], [2, 3, 4])).toEqual([2, 3]);

    // Test with three arrays
    expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5])).toEqual([3]);

    // Test with four arrays (including duplicates)
    expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5], [3, 4, 5])).toEqual(
      [3],
    );
  });

  it("should handle empty arrays", () => {
    expect(getArraysCommon([], [])).toEqual([]);
    expect(getArraysCommon([1, 2, 3], [])).toEqual([]);
    expect(getArraysCommon([], [1, 2, 3])).toEqual([]);
  });

  it("should handle arrays with no common elements", () => {
    expect(getArraysCommon([1, 2, 3], [4, 5, 6])).toEqual([]);
    expect(getArraysCommon([1, 2], [3, 4], [5, 6])).toEqual([]);
  });

  it("should handle arrays of strings", () => {
    expect(getArraysCommon(["a", "b", "c"], ["b", "c", "d"])).toEqual([
      "b",
      "c",
    ]);
    expect(
      getArraysCommon(["a", "b", "c"], ["b", "c", "d"], ["c", "d", "e"]),
    ).toEqual(["c"]);
  });

  it("should handle arrays of objects", () => {
    const obj1 = { id: 1 };
    const obj2 = { id: 2 };
    const obj3 = { id: 3 };

    expect(getArraysCommon([obj1, obj2, obj3], [obj2, obj3], [obj3])).toEqual([
      obj3,
    ]);
  });
});
