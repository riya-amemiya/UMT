import { mergeSort } from "@/Array/mergeSort";

describe("mergeSort", () => {
  it("空の配列を渡した場合、空の配列を返す", () => {
    expect(mergeSort([])).toEqual([]);
  });

  it("既にソートされた配列を渡した場合、同じ配列を返す", () => {
    expect(mergeSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("逆順にソートされた配列を渡した場合、正しくソートされた配列を返す", () => {
    expect(mergeSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("ランダムな順序の配列を渡した場合、正しくソートされた配列を返す", () => {
    expect(mergeSort([2, 3, 1])).toEqual([1, 2, 3]);
  });

  it("重複した要素を含む配列を渡した場合、正しくソートされた配列を返す", () => {
    expect(mergeSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });
});
