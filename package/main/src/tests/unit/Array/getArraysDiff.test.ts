import { getArraysDiff } from "@/Array/getArraysDiff";

describe("getArraysDiff関数のテスト", () => {
  // 通常のケース
  test("通常の配列", () => {
    expect(getArraysDiff([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual([1, 5]);
    expect(getArraysDiff([-1, 0, 1], [0, 1, 2], [1, 2, 3])).toEqual([-1, 3]);
    expect(
      getArraysDiff([0.1, 0.2, 0.3], [0.2, 0.3, 0.4], [0.3, 0.4, 0.5]),
    ).toEqual([0.1, 0.5]);
    expect(
      getArraysDiff(["a", "b", "c"], ["b", "c", "d"], ["c", "d", "e"]),
    ).toEqual(["a", "e"]);
    expect(
      getArraysDiff(["あ", "い", "う"], ["い", "う", "え"], ["う", "え", "お"]),
    ).toEqual(["あ", "お"]);
    expect(getArraysDiff([true, false], [false, true], [true, false])).toEqual(
      [],
    );
    expect(
      getArraysDiff([1, "a", true], [2, "b", false], [3, "c", true]),
    ).toEqual([1, "a", 2, "b", false, 3, "c"]);
  });

  // 空の配列があるケース
  test("空の配列", () => {
    expect(getArraysDiff([], [1, 2], [2, 3])).toEqual([1, 3]);
    expect(getArraysDiff([1, 2], [], [2, 3])).toEqual([1, 3]);
    expect(getArraysDiff([1, 2], [2, 3], [])).toEqual([1, 3]);
    expect(getArraysDiff([], [], [1, 2, 3])).toEqual([1, 2, 3]);
    expect(getArraysDiff(["a", "b"], [], ["b", "c"])).toEqual(["a", "c"]);
    expect(getArraysDiff([], [true, false], [false, null])).toEqual([
      true,
      null,
    ]);
    expect(getArraysDiff([{ x: 1 }], [], [{ z: 3 }])).toEqual([
      { x: 1 },
      { z: 3 },
    ]);
    expect(getArraysDiff([], [[1, 2]], [[3, 4]])).toEqual([
      [1, 2],
      [3, 4],
    ]);
    expect(getArraysDiff([1, "a", true], [], [3, "c", false])).toEqual([
      1,
      "a",
      true,
      3,
      "c",
      false,
    ]);
    expect(getArraysDiff([], [], [])).toEqual([]);
  });

  // 重複要素がないケース
  test("重複なし", () => {
    expect(getArraysDiff([1, 2], [3, 4], [5, 6])).toEqual([1, 2, 3, 4, 5, 6]);
    expect(getArraysDiff(["a", "b"], ["c", "d"], ["e", "f"])).toEqual([
      "a",
      "b",
      "c",
      "d",
      "e",
      "f",
    ]);
    expect(getArraysDiff([true], [false], [null])).toEqual([true, false, null]);
    expect(getArraysDiff([{ x: 1 }], [{ y: 2 }], [{ z: 3 }])).toEqual([
      { x: 1 },
      { y: 2 },
      { z: 3 },
    ]);
    expect(getArraysDiff([[1]], [[2]], [[3]])).toEqual([[1], [2], [3]]);
    expect(getArraysDiff([1, "a"], [2, "b"], [3, "c"])).toEqual([
      1,
      "a",
      2,
      "b",
      3,
      "c",
    ]);
    expect(getArraysDiff([0.1], [0.2], [0.3])).toEqual([0.1, 0.2, 0.3]);
    expect(
      getArraysDiff(
        [new Date("2021-01-01")],
        [new Date("2021-01-02")],
        [new Date("2021-01-03")],
      ),
    ).toEqual([
      new Date("2021-01-01"),
      new Date("2021-01-02"),
      new Date("2021-01-03"),
    ]);
    expect(getArraysDiff([/a/], [/b/], [/c/])).toEqual([/a/, /b/, /c/]);
  });

  // 全ての配列が同じ要素を持つケース
  test("全配列で重複", () => {
    expect(getArraysDiff([1, 2, 3], [1, 2, 3], [1, 2, 3])).toEqual([]);
    expect(
      getArraysDiff(["a", "b", "c"], ["a", "b", "c"], ["a", "b", "c"]),
    ).toEqual([]);
    expect(getArraysDiff([true, false], [true, false], [true, false])).toEqual(
      [],
    );
    expect(
      getArraysDiff([1, "a", true], [1, "a", true], [1, "a", true]),
    ).toEqual([]);
    expect(getArraysDiff([0.1, 0.2], [0.1, 0.2], [0.1, 0.2])).toEqual([]);
  });

  // 全ての配列が同じ要素を持っているが、順序が異なるケース
  test("全配列で重複(順序異なる)", () => {
    expect(getArraysDiff([1, 2, 3], [3, 1, 2], [2, 3, 1])).toEqual([]);
    expect(
      getArraysDiff(["a", "b", "c"], ["c", "a", "b"], ["b", "c", "a"]),
    ).toEqual([]);
    expect(
      getArraysDiff(
        [true, false, null],
        [null, true, false],
        [false, null, true],
      ),
    ).toEqual([]);
    expect(
      getArraysDiff([1, "a", true], [true, 1, "a"], ["a", true, 1]),
    ).toEqual([]);
    expect(
      getArraysDiff([0.1, 0.2, 0.3], [0.3, 0.1, 0.2], [0.2, 0.3, 0.1]),
    ).toEqual([]);
  });

  // 型が異なるケース
  test("型が異なる", () => {
    expect(getArraysDiff([1, "a", true], [true, "a"], [1, 2])).toEqual([2]);
    expect(
      getArraysDiff(
        [{ x: 1 }, [1, 2], 3],
        ["a", true, null],
        [false, 5, { y: 2 }],
      ),
    ).toEqual([{ x: 1 }, [1, 2], 3, "a", true, null, false, 5, { y: 2 }]);
    expect(
      getArraysDiff([1, "2", true], ["1", 2, false], [true, "1", 2]),
    ).toEqual([1, "2", false]);
    expect(getArraysDiff([0, ""], [false, null], [undefined, NaN])).toEqual([
      0,
      "",
      false,
      null,
      undefined,
      NaN,
    ]);
    expect(getArraysDiff([1n, 2n], [BigInt(1), BigInt(2)], [1, 2])).toEqual([
      1, 2,
    ]);
    expect(getArraysDiff([new Map()], [new Set()], [new WeakMap()])).toEqual([
      new Map(),
      new Set(),
      new WeakMap(),
    ]);
    expect(
      getArraysDiff(
        [Symbol.iterator],
        [Symbol.asyncIterator],
        [Symbol.toStringTag],
      ),
    ).toEqual([Symbol.iterator, Symbol.asyncIterator, Symbol.toStringTag]);
  });
});
