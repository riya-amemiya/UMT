import { drop } from "@/Array/drop";

describe("drop関数のテスト", () => {
  it("配列の先頭からn個の要素を除外する", () => {
    expect(drop([1, 2, 3, 4, 5], 2)).toEqual([3, 4, 5]);
    expect(drop([1, 2, 3, 4, 5])).toEqual([2, 3, 4, 5]);
  });

  it("directionが'left'の場合、配列の先頭からn個の要素を除外する", () => {
    expect(drop([1, 2, 3, 4, 5], 2, "left")).toEqual([3, 4, 5]);
  });

  it("directionが'right'の場合、配列の末尾からn個の要素を除外する", () => {
    expect(drop([1, 2, 3, 4, 5], 2, "right")).toEqual([1, 2, 3]);
  });

  it("directionが'between'の場合、配列の中央からn個の要素を除外する", () => {
    expect(drop([1, 2, 3, 4, 5], 1, "between")).toEqual([1, 2, 4, 5]);
    expect(drop([1, 2, 3, 4, 5, 6], 2, "between")).toEqual([1, 2, 5, 6]);
  });

  it("nが配列の長さ以上の場合、空の配列を返す", () => {
    expect(drop([1, 2, 3], 4)).toEqual([]);
    expect(drop([1, 2, 3], 3)).toEqual([]);
  });

  it("nが0の場合、元の配列をそのまま返す", () => {
    expect(drop([1, 2, 3], 0)).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "left")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "right")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], 0, "between")).toEqual([1, 2, 3]);
  });

  it("nが負の値の場合、元の配列をそのまま返す", () => {
    expect(drop([1, 2, 3], -1)).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -2, "left")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -3, "right")).toEqual([1, 2, 3]);
    expect(drop([1, 2, 3], -4, "between")).toEqual([1, 2, 3]);
  });

  it("空の配列が渡された場合、空の配列を返す", () => {
    expect(drop([], 1)).toEqual([]);
    expect(drop([], 2, "left")).toEqual([]);
    expect(drop([], 3, "right")).toEqual([]);
    expect(drop([], 4, "between")).toEqual([]);
  });

  it("directionに無効な値が渡された場合、leftとして扱う", () => {
    // @ts-ignore
    expect(drop([1, 2, 3], 1, "invalid")).toEqual([2, 3]);
  });
});
