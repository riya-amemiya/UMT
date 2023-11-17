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
});
