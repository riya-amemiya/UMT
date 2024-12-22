import { zipLongest } from "@/Array/zipLongest";

describe("zipLongest関数のテスト", () => {
  it("異なる長さの配列を組み合わせる", () => {
    const array1 = [1, 2];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true, false];
    const result = zipLongest(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [undefined, "three", true],
      [undefined, undefined, false],
    ]);
  });

  it("すべての配列が同じ長さの場合", () => {
    const array1 = [1, 2, 3];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true];
    const result = zipLongest(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [3, "three", true],
    ]);
  });

  it("空の配列を含む組み合わせ", () => {
    const array1 = [1, 2, 3];
    // @ts-ignore
    const array2 = [];
    // @ts-ignore
    const result = zipLongest(array1, array2);
    expect(result).toEqual([
      [1, undefined],
      [2, undefined],
      [3, undefined],
    ]);
  });

  it("引数がない場合は空の配列を返す", () => {
    const result = zipLongest();
    expect(result).toEqual([]);
  });
});
