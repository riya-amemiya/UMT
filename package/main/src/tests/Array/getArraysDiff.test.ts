import { getArraysDiff } from "@/Array/getArraysDiff";

describe("getArraysDiff関数のテスト", () => {
  // 通常のケース
  test("通常の配列", () => {
    expect(getArraysDiff([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual([1, 5]);
  });

  // 空の配列があるケース
  test("空の配列", () => {
    expect(getArraysDiff([1, 2, 3], [], [3, 4, 5])).toEqual([1, 2, 4, 5]);
  });

  // 重複要素がないケース
  test("重複なし", () => {
    expect(getArraysDiff([1, 2], [3, 4], [5, 6])).toEqual([1, 2, 3, 4, 5, 6]);
  });

  // 全ての配列が同じ要素を持つケース
  test("全配列で重複", () => {
    expect(getArraysDiff([1, 2, 3], [1, 2, 3], [1, 2, 3])).toEqual([]);
  });

  // 全ての配列が同じ要素を持っているが、順序が異なるケース
  test("全配列で重複(順序異なる)", () => {
    expect(getArraysDiff([1, 2, 3], [3, 1, 2], [2, 3, 1])).toEqual([]);
  });

  // 型が異なるケース
  test("型が異なる", () => {
    expect(getArraysDiff([1, "a", true], [true, "a"], [1, 2])).toEqual([2]);
  });
});
