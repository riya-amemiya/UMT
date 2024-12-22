import { mathSeparator } from "@/Math/mathSeparator";
describe("mathSeparator", () => {
  // 数字の型（number）のテスト
  it("should handle single digit numbers", () => {
    expect(mathSeparator(5)).toEqual([5, 0]);
  });

  it("should handle multi-digit numbers", () => {
    expect(mathSeparator(1250)).toEqual([1000, 250]);
  });

  // 文字列型の数字のテスト
  it("should handle single digit string numbers", () => {
    expect(mathSeparator("5")).toEqual([5, 0]);
  });

  it("should handle multi-digit string numbers", () => {
    expect(mathSeparator("1250")).toEqual([1000, 250]);
  });

  // 0および'0'のケース
  it("should handle 0", () => {
    expect(mathSeparator(0)).toEqual([0, 0]);
  });

  it('should handle "0"', () => {
    expect(mathSeparator("0")).toEqual([0, 0]);
  });

  // 不正な値のケース
  it("should return [0, 0] for non-number input", () => {
    expect(mathSeparator("abc")).toEqual([0, 0]);
  });

  // 小数のケース
  it("should handle decimal numbers", () => {
    expect(mathSeparator(12.5)).toEqual([10, 2.5]);
  });

  it("should handle decimal string numbers", () => {
    expect(mathSeparator("12.5")).toEqual([10, 2.5]);
  });
});
