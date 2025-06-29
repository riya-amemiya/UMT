import { arraysJoin } from "@/Array/arraysJoin";

describe("arraysJoin", () => {
  test("should join arrays without duplicates", () => {
    // Basic array joining
    expect(arraysJoin([1, 2, 3], [4, 5, 6])).toEqual([1, 2, 3, 4, 5, 6]);

    // Multiple arrays joining
    expect(arraysJoin([1, 2, 3], [4, 5, 6], [7, 8, 9])).toEqual([
      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ]);

    // Arrays with duplicates
    expect(arraysJoin([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual([
      1, 2, 3, 4, 5,
    ]);

    // Empty array cases
    expect(arraysJoin([], [1, 2, 3])).toEqual([1, 2, 3]);
    expect(arraysJoin([1, 2, 3], [])).toEqual([1, 2, 3]);
  });

  test("should handle arrays with string elements", () => {
    expect(arraysJoin(["a", "b", "c"], ["b", "c", "d"])).toEqual([
      "a",
      "b",
      "c",
      "d",
    ]);
    expect(
      arraysJoin(["hello", "world"], ["world", "foo"], ["bar", "hello"]),
    ).toEqual(["hello", "world", "foo", "bar"]);
  });

  test("should handle arrays with mixed types", () => {
    expect(arraysJoin([1, "a", true], [2, "a", false])).toEqual([
      1,
      "a",
      true,
      2,
      false,
    ]);
    expect(
      arraysJoin([null, undefined, 0], [0, "", false], [null, Number.NaN]),
    ).toEqual([null, undefined, 0, "", false, Number.NaN]);
  });

  test("should handle arrays with objects", () => {
    const obj1 = { id: 1 };
    const obj2 = { id: 2 };
    const obj3 = { id: 1 };

    expect(arraysJoin([obj1, obj2], [obj2, obj3])).toEqual([obj1, obj2, obj3]);

    expect(arraysJoin([obj1, obj2], [obj1, obj2])).toEqual([obj1, obj2]);
  });

  test("should handle special numeric values", () => {
    expect(
      arraysJoin(
        [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY],
        [Number.NaN, -0, 0],
      ),
    ).toEqual([
      Number.NaN,
      Number.POSITIVE_INFINITY,
      Number.NEGATIVE_INFINITY,
      0,
    ]);

    expect(arraysJoin([Number.NaN], [Number.NaN])).toHaveLength(1);
  });

  test("should handle arrays with symbols", () => {
    const sym1 = Symbol("test");
    const sym2 = Symbol("test");
    const sym3 = Symbol.for("shared");

    expect(arraysJoin([sym1, sym2], [sym2, sym3])).toEqual([sym1, sym2, sym3]);

    expect(arraysJoin([sym1], [sym1])).toEqual([sym1]);
  });

  test("should handle large arrays efficiently", () => {
    const largeArray1 = Array.from({ length: 10_000 }, (_, i) => i);
    const largeArray2 = Array.from({ length: 10_000 }, (_, i) => i + 5000);
    const result = arraysJoin(largeArray1, largeArray2);

    expect(result).toHaveLength(15_000);
    expect(result[0]).toBe(0);
    expect(result[result.length - 1]).toBe(14_999);
  });

  test("should handle nested arrays as elements", () => {
    const nestedArray1 = [
      [1, 2],
      [3, 4],
    ];
    const nestedArray2 = [
      [3, 4],
      [5, 6],
    ];
    const sharedRef = [3, 4];

    expect(arraysJoin(nestedArray1, nestedArray2)).toHaveLength(4);

    expect(arraysJoin([sharedRef], [sharedRef])).toEqual([sharedRef]);
  });

  test("should handle single array input", () => {
    expect(arraysJoin([1, 2, 3])).toEqual([1, 2, 3]);
    expect(arraysJoin([1, 1, 2, 2, 3, 3])).toEqual([1, 2, 3]);
  });

  test("should handle no additional arrays", () => {
    const result = arraysJoin([1, 2, 2, 3, 3, 3]);
    expect(result).toEqual([1, 2, 3]);
  });

  test("should preserve type through generic parameter", () => {
    const numberResult = arraysJoin<number[]>([1, 2], [3, 4]);
    const stringResult = arraysJoin<string[]>(["a", "b"], ["c", "d"]);

    expect(numberResult).toEqual([1, 2, 3, 4]);
    expect(stringResult).toEqual(["a", "b", "c", "d"]);
  });

  test("should handle Date objects", () => {
    const date1 = new Date("2024-01-01");
    const date2 = new Date("2024-01-02");
    const date3 = new Date("2024-01-01");

    expect(arraysJoin([date1, date2], [date2, date3])).toEqual([
      date1,
      date2,
      date3,
    ]);
  });

  test("should handle regular expressions", () => {
    const regex1 = /test/g;
    const regex2 = /test/i;
    const regex3 = /test/g;

    expect(arraysJoin([regex1, regex2], [regex2, regex3])).toEqual([
      regex1,
      regex2,
      regex3,
    ]);
  });

  test("should handle functions", () => {
    const fn1 = () => 1;
    const fn2 = () => 2;
    const fn3 = () => 3;

    expect(arraysJoin([fn1, fn2], [fn2, fn3])).toEqual([fn1, fn2, fn3]);

    expect(arraysJoin([fn1], [fn1])).toEqual([fn1]);
  });

  test("should maintain insertion order", () => {
    expect(arraysJoin([3, 1, 2], [2, 4, 1])).toEqual([3, 1, 2, 4]);
    expect(arraysJoin(["c", "a", "b"], ["b", "d", "a"])).toEqual([
      "c",
      "a",
      "b",
      "d",
    ]);
  });
});
