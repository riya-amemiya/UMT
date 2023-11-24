import { shuffle2DArray } from "@/Array/shuffle2DArray";

describe("shuffle2DArray関数のテスト", () => {
  it("2次元配列全体がシャッフルされること", () => {
    const array = [
      [1, 2],
      [3, 4],
      [5, 6],
    ];
    const shuffledArray = shuffle2DArray(array);
    // 全体の配列がシャッフルされているか確認
    expect(shuffledArray.map((subArray) => subArray.sort())).not.toEqual(
      array.map((subArray) => subArray.sort()),
    );
  });

  it("空の2次元配列がそのまま返されること", () => {
    const array: number[][] = [];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray).toEqual(array);
  });

  it("サブ配列が空の場合もそのまま返されること", () => {
    const array = [[], [], []];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray).toEqual(array);
  });

  it("シャッフルされた2次元配列の長さが元の配列と同じであること", () => {
    const array = [
      [1, 2],
      [3, 4],
      [5, 6],
    ];
    const shuffledArray = shuffle2DArray(array);
    expect(shuffledArray.length).toBe(array.length);
    shuffledArray.forEach((subArray, index) => {
      expect(subArray.length).toBe(array[index].length);
    });
  });

  it("異なる型を含む2次元配列がシャッフルされること", () => {
    const array = [
      [1, "2"],
      ["3", 4],
      [5, "6"],
    ];
    const shuffledArray = shuffle2DArray(array);
    // 全体の配列がシャッフルされているか確認
    expect(shuffledArray.map((subArray) => subArray.sort())).not.toEqual(
      array.map((subArray) => subArray.sort()),
    );
  });

  it("長い配列の要素がシャッフルされること", () => {
    const array = Array.from({ length: 1000 }, (_, index) => [
      index,
      index + 1,
    ]);
    const shuffledArray = shuffle2DArray(array);
    // 全体の配列がシャッフルされているか確認
    expect(shuffledArray.map((subArray) => subArray.sort())).not.toEqual(
      array.map((subArray) => subArray.sort()),
    );
  });
});
