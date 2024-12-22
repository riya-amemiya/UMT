import { deviationValue } from "@/Math/deviationValue";

// 偏差値の計算に関するテスト
describe("deviationValue", () => {
  // 正常な入力値でのテスト
  test("正常な入力値", () => {
    expect(deviationValue(100, 50, 10)).toBe(100);
  });

  // 平均値と同じ値でのテスト
  test("平均値と同じ値", () => {
    expect(deviationValue(50, 50, 10)).toBe(50);
  });

  // 標準偏差が0のケース（例外処理が必要な場合）
  test("標準偏差が0", () => {
    expect(deviationValue(100, 50, 0)).toBe(Infinity);
  });

  // 負の値を含むケース
  test("負の値を含む", () => {
    expect(deviationValue(-20, 0, 20)).toBe(40);
  });

  // 浮動小数点数を含むケース
  test("浮動小数点数を含む", () => {
    expect(deviationValue(55.5, 50, 10)).toBeCloseTo(55.5);
  });
});
