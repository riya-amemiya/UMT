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

  it("should handle arrays with mixed types", () => {
    expect(
      getArraysCommon([1, "a", true], ["a", true, 2], [true, "a", 3]),
    ).toEqual(["a", true]);
    expect(
      getArraysCommon([null, 0, false], [0, false, ""], [false, 0, undefined]),
    ).toEqual([0, false]);
  });

  it("should handle arrays with special numeric values", () => {
    expect(
      getArraysCommon(
        [Number.NaN, Number.POSITIVE_INFINITY, -0],
        [Number.POSITIVE_INFINITY, -0, 1],
        [-0, 1, 2],
      ),
    ).toEqual([-0]);

    expect(getArraysCommon([Number.NaN, 1], [Number.NaN, 1])).toEqual([
      Number.NaN,
      1,
    ]);
  });

  it("should handle arrays with duplicate elements", () => {
    expect(
      getArraysCommon([1, 1, 2, 2, 3], [2, 2, 3, 3, 4], [3, 3, 4, 4, 5]),
    ).toEqual([3]);
    expect(getArraysCommon([1, 2, 2, 3], [2, 3, 3, 4], [3, 4, 4, 5])).toEqual([
      3,
    ]);
  });

  it("should handle arrays with symbols", () => {
    const sym1 = Symbol("test1");
    const sym2 = Symbol("test2");
    const sym3 = Symbol.for("shared");

    expect(getArraysCommon([sym1, sym2, sym3], [sym2, sym3], [sym3])).toEqual([
      sym3,
    ]);

    const symA = Symbol("same");
    const symB = Symbol("same");
    expect(getArraysCommon([symA, symB], [symA, symB])).toEqual([symA, symB]);
  });

  it("should handle single array input", () => {
    expect(getArraysCommon([1, 2, 3])).toEqual([1, 2, 3]);
    expect(getArraysCommon(["a", "b", "c"])).toEqual(["a", "b", "c"]);
    expect(getArraysCommon([])).toEqual([]);
  });

  it("should handle large arrays efficiently", () => {
    const array1 = Array.from({ length: 10_000 }, (_, i) => i);
    const array2 = Array.from({ length: 10_000 }, (_, i) => i + 5000);
    const array3 = Array.from({ length: 10_000 }, (_, i) => i + 7500);

    const result = getArraysCommon<number[]>(array1, array2, array3);

    expect(result).toHaveLength(2500);
    expect(result[0]).toBe(7500);
    expect(result[result.length - 1]).toBe(9999);
  });

  it("should handle nested arrays as elements", () => {
    const nestedArray1 = [1, 2];
    const nestedArray2 = [3, 4];
    const nestedArray3 = [5, 6];

    expect(
      getArraysCommon(
        [nestedArray1, nestedArray2],
        [nestedArray2, nestedArray3],
        [nestedArray2],
      ),
    ).toEqual([nestedArray2]);

    expect(
      getArraysCommon(
        [
          [1, 2],
          [3, 4],
        ],
        [
          [1, 2],
          [3, 4],
        ],
      ),
    ).toEqual([]);
  });

  it("should handle Date objects", () => {
    const date1 = new Date("2024-01-01");
    const date2 = new Date("2024-01-02");
    const date3 = new Date("2024-01-03");

    expect(
      getArraysCommon([date1, date2, date3], [date2, date3], [date3]),
    ).toEqual([date3]);

    const dateA = new Date("2024-01-01");
    const dateB = new Date("2024-01-01");
    expect(getArraysCommon([dateA, dateB], [dateA, dateB])).toEqual([
      dateA,
      dateB,
    ]);
  });

  it("should handle regular expressions", () => {
    const regex1 = /test/g;
    const regex2 = /test/i;
    const regex3 = /hello/;

    expect(
      getArraysCommon([regex1, regex2, regex3], [regex2, regex3], [regex3]),
    ).toEqual([regex3]);
  });

  it("should handle functions", () => {
    const fn1 = () => 1;
    const fn2 = () => 2;
    const fn3 = () => 3;

    expect(getArraysCommon([fn1, fn2, fn3], [fn2, fn3], [fn3])).toEqual([fn3]);

    expect(getArraysCommon([fn1, fn2], [fn1, fn2])).toEqual([fn1, fn2]);
  });

  it("should preserve order from first array", () => {
    expect(getArraysCommon([3, 1, 2], [2, 3, 1], [1, 3, 2])).toEqual([3, 1, 2]);
    expect(
      getArraysCommon(["c", "a", "b"], ["b", "c", "a"], ["a", "c", "b"]),
    ).toEqual(["c", "a", "b"]);
  });

  it("should handle boolean values", () => {
    expect(getArraysCommon([true, false, true], [false, true], [true])).toEqual(
      [true],
    );
    expect(
      getArraysCommon([true, false], [false, true], [true, false]),
    ).toEqual([true, false]);
  });
});
