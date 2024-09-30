import { quickSortSimple } from "@/Simple/Array/quickSortSimple";

describe("quickSortSimple", () => {
  it("空の配列をソートすると空の配列が返される", () => {
    expect(quickSortSimple([])).toEqual([]);
  });
  it("startIDが配列の範囲外の場合", () => {
    expect(quickSortSimple([3, 1, 4], undefined, -1, 2)).toEqual([1, 3, 4]);
    expect(quickSortSimple([3, 1, 4], undefined, 4, 2)).toEqual([1, 3, 4]);
  });

  it("endIDが配列の範囲外の場合", () => {
    expect(quickSortSimple([3, 1, 4], undefined, 0, 5)).toEqual([1, 3, 4]);
  });

  it("startIDがendIDより大きい場合", () => {
    expect(quickSortSimple([3, 1, 4], undefined, 2, 1)).toEqual([1, 3, 4]);
  });
});
