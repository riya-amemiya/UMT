import { shuffle } from "@/Array/shuffle";

describe("shuffle関数のテスト", () => {
  it("配列の要素がシャッフルされること", () => {
    const array = [1, 2, 3, 4, 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
  });

  it("空の配列がそのまま返されること", () => {
    const array: number[] = [];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).toEqual(array);
  });

  it("シャッフルされた配列の長さが元の配列と同じであること", () => {
    const array = [1, 2, 3, 4, 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray.length).toBe(array.length);
  });

  it("文字列と数値が混在した配列の要素がシャッフルされること", () => {
    const array = [1, "2", 3, "4", 5];
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
  });

  it("長い配列の要素がシャッフルされること", () => {
    const array = Array.from({ length: 1000 }, (_, index) => index);
    const shuffledArray = shuffle(array);
    expect(shuffledArray).not.toEqual(array);
    expect(shuffledArray.sort()).toEqual(array.sort());
  });
});
