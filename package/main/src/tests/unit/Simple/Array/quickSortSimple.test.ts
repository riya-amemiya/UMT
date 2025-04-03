import { quickSortSimple } from "@/Simple/Array/quickSortSimple";

describe("quickSortSimple", () => {
  it("returns empty array when sorting empty array", () => {
    expect(quickSortSimple([])).toEqual([]);
  });
  it("handles startID outside array bounds", () => {
    expect(quickSortSimple([3, 1, 4], undefined, -1, 2)).toEqual([1, 3, 4]);
    expect(quickSortSimple([3, 1, 4], undefined, 4, 2)).toEqual([1, 3, 4]);
  });

  it("handles endID outside array bounds", () => {
    expect(quickSortSimple([3, 1, 4], undefined, 0, 5)).toEqual([1, 3, 4]);
  });

  it("handles startID greater than endID", () => {
    expect(quickSortSimple([3, 1, 4], undefined, 2, 1)).toEqual([1, 3, 4]);
  });
});
