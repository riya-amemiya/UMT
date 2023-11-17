import { median } from "@/Math/median";

describe("median", () => {
  it("偶数個の要素を持つ配列の中央値を計算する", () => {
    expect(median([1, 2, 3, 4])).toBe(2.5);
  });

  it("奇数個の要素を持つ配列の中央値を計算する", () => {
    expect(median([1, 3, 3, 6, 7, 8, 9])).toBe(6);
  });

  it("空の配列が与えられた場合はNaNを返す", () => {
    expect(median([])).toBeNaN();
  });

  it("ソートされていない配列の中央値を計算する", () => {
    expect(median([9, 1, 5, 3, 6])).toBe(5);
  });
});
