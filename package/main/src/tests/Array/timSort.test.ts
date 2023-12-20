import { timSort } from "@/Array/timSort";

describe("timSort", () => {
  it("空の配列をソートすると空の配列が返される", () => {
    expect(timSort([])).toEqual([]);
  });

  it("既にソートされた配列はそのまま返される", () => {
    expect(timSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("逆順にソートされた配列を正しくソートする", () => {
    expect(timSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("ランダムな配列を正しくソートする", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    const sortedArray = [...array].sort((a, b) => a - b);
    expect(timSort(array)).toEqual(sortedArray);
  });

  it("重複した要素を含む配列を正しくソートする", () => {
    expect(timSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });

  it("大量の要素を含む配列を正しくソートする", () => {
    const largeArray = Array.from({ length: 10000 }, () =>
      Math.floor(Math.random() * 10000),
    );
    const sortedArray = [...largeArray].sort((a, b) => a - b);
    expect(timSort(largeArray)).toEqual(sortedArray);
  });

  it("負の数を含む配列を正しくソートする", () => {
    expect(timSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("配列を降順にソートする", () => {
    expect(timSort([1, 2, 3], (a, b) => b - a)).toEqual([3, 2, 1]);
  });

  it("部分配列をソートする", () => {
    expect(timSort([1, 3, 2, 5, 4], (a, b) => a - b, 1, 3)).toEqual([
      1, 2, 3, 5, 4,
    ]);
  });
});
