import { quickSort } from "@/Array/quickSort";
describe("quickSort", () => {
  it("空の配列をソートすると空の配列が返される", () => {
    expect(quickSort([])).toEqual([]);
  });

  it("既にソートされた配列はそのまま返される", () => {
    expect(quickSort([1, 2, 3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
  });

  it("逆順にソートされた配列を正しくソートする", () => {
    expect(quickSort([5, 4, 3, 2, 1])).toEqual([1, 2, 3, 4, 5]);
  });

  it("ランダムな配列を正しくソートする", () => {
    expect(quickSort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5])).toEqual([
      1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9,
    ]);
  });

  it("重複した要素を含む配列を正しくソートする", () => {
    expect(quickSort([3, 3, 3, 2, 1, 1, 4, 4, 5])).toEqual([
      1, 1, 2, 3, 3, 3, 4, 4, 5,
    ]);
  });

  it("大量の要素を含む配列を正しくソートする", () => {
    const largeArray = Array.from({ length: 10000 }, () =>
      Math.floor(Math.random() * 10000),
    );
    const sortedArray = [...largeArray].sort((a, b) => a - b);
    expect(quickSort(largeArray)).toEqual(sortedArray);
  });

  it("負の数を含む配列を正しくソートする", () => {
    expect(quickSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("配列を降順にソートする", () => {
    expect(quickSort([1, 2, 3, 4, 5], (a, b) => b - a)).toEqual([
      5, 4, 3, 2, 1,
    ]);
  });

  it("部分配列をソートする", () => {
    expect(quickSort([3, 1, 4, 1, 5], undefined, 1, 3)).toEqual([
      3, 1, 1, 4, 5,
    ]);
  });

  it("単一要素の配列が変更されずに返される", () => {
    expect(quickSort([1])).toEqual([1]);
  });

  it("compareFunctionが例外を投げる場合", () => {
    const throwCompare = () => {
      throw new Error("Error during comparison");
    };
    expect(() => quickSort([3, 1, 4], throwCompare)).toThrow(
      "Error during comparison",
    );
  });
});
