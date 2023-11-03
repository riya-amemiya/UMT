import { division } from "@/Math/division";

describe("division function", () => {
  // 整数に対するテスト
  it("should return the correct division for integers", () => {
    expect(division(10, 2)).toBe(5);
    expect(division(30, 5)).toBe(6);
  });

  // isFloor=true の場合
  it("should return floored division if isFloor is true", () => {
    expect(division(7, 2, true)).toBe(3.5);
    expect(division(3, 5, true)).toBe(0.6);
  });

  // isFloor=false の場合
  it("should return division and remainder if isFloor is false", () => {
    expect(division(10, 3, false)).toEqual([3, 1]);
    expect(division(7, 2, false)).toEqual([3, 1]);
    expect(division(3, 5, false)).toEqual([0, 3]);
  });

  // 小数に対するテスト
  it("should handle decimals correctly", () => {
    expect(division(10.5, 2.1)).toBe(5);
  });

  // 精度チェック
  it("should handle precision correctly", () => {
    expect(division(0.1, 0.2)).toBeCloseTo(0.5);
  });
});
