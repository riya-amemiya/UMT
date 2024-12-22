import { randomSelect } from "@/Array/randomSelect";

describe("randomSelect", () => {
  it("指定された数の要素をランダムに選択する", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, 2);
    expect(result).toHaveLength(2);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("選択する要素の数が配列の長さを超える場合、元の配列の長さと同じ数の要素を返す", () => {
    const array = [1, 2, 3];
    const result = randomSelect(array, 5);
    expect(result).toHaveLength(3);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("空の配列を渡した場合、空の配列を返す", () => {
    const result = randomSelect([], 3);
    expect(result).toEqual([]);
  });

  it("重複を許す場合、指定された数の要素を返す", () => {
    const array = [1, 2, 3];
    const result = randomSelect(array, 5, true);
    expect(result).toHaveLength(5);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("重複を許さない場合、指定された数の要素を返す", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, 3, false);
    expect(result).toHaveLength(3);
    result.forEach((item) => {
      expect(array).toContain(item);
    });
  });

  it("選択する要素の数がマイナスの場合、空の配列を返す", () => {
    const array = [1, 2, 3, 4, 5];
    const result = randomSelect(array, -1);
    expect(result).toEqual([]);
  });
});
