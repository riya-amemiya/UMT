import { range } from "@/Array/range";

describe("range", () => {
  test("-5から5までの連続した数字の配列を生成する。", () => {
    const result = range(-5, 5);
    expect(result).toEqual([-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]);
  });

  test("0から0までの連続した数字の配列を生成する。", () => {
    const result = range(0, 0);
    expect(result).toEqual([0]);
  });

  test("0から5までの連続した数字の配列を生成する。", () => {
    const result = range(1, 10);

    expect(result).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  });

  test("5から5までの連続した数字の配列を生成する。", () => {
    const result = range(5, 5);
    expect(result).toEqual([5]);
  });

  test("0から5までの連続した数字の配列を生成する。", () => {
    const result = range(5);
    expect(result).toEqual([0, 1, 2, 3, 4, 5]);
  });
});
