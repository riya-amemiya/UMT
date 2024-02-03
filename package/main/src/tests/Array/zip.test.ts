import { zip } from "@/Array/zip";

describe("zip関数のテスト", () => {
  it("複数の配列を組み合わせる", () => {
    const array1 = [1, 2, 3];
    const array2 = ["one", "two", "three"];
    const array3 = [true, false, true];
    const result = zip(array1, array2, array3);
    expect(result).toEqual([
      [1, "one", true],
      [2, "two", false],
      [3, "three", true],
    ]);
  });

  it("異なる長さの配列を組み合わせる", () => {
    const array1 = [1, 2];
    const array2 = ["one", "two", "three"];
    const result = zip(array1, array2);
    expect(result).toEqual([
      [1, "one"],
      [2, "two"],
    ]);
  });

  it("空の配列を含む組み合わせ", () => {
    const array1 = [1, 2, 3];
    // @ts-ignore
    const array2 = [];
    // @ts-ignore
    const result = zip(array1, array2);
    expect(result).toEqual([]);
  });

  it("引数がない場合は空の配列を返す", () => {
    const result = zip();
    expect(result).toEqual([]);
  });
});
