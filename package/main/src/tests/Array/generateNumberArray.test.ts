import { generateNumberArray } from "@/Array/generateNumberArray";

describe("generateNumberArray", () => {
  it("指定された長さの配列を生成する", () => {
    const result = generateNumberArray(5);
    expect(result).toEqual([0, 1, 2, 3, 4]);
  });

  it("最小値と最大値を指定して配列を生成する", () => {
    const result = generateNumberArray(5, 10, 14);
    expect(result).toEqual([10, 11, 12, 13, 14]);
  });

  it("長さが0の場合、空の配列を返す", () => {
    const result = generateNumberArray(0);
    expect(result).toEqual([]);
  });

  it("最小値が最大値より大きい場合、エラーを投げる", () => {
    expect(() => generateNumberArray(5, 10, 5)).toThrow(
      "min should be less than or equal to max",
    );
  });

  it("長さが1の場合、最小値を返す", () => {
    const result = generateNumberArray(1, 10, 20);
    expect(result).toEqual([10]);
  });

  it("ランダムな値で配列を生成する", () => {
    const result = generateNumberArray(5, 10, 14, true);
    expect(result).toHaveLength(5);
    result.forEach((item) => {
      expect(item).toBeGreaterThanOrEqual(10);
      expect(item).toBeLessThanOrEqual(14);
    });
  });
});
