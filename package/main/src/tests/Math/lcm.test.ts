import { lcm } from "@/Math/lcm";
describe("lcm関数のテスト", () => {
  // 正数同士のテスト
  test("正数の入力", () => {
    expect(lcm(4, 5)).toBe(20);
  });

  // 0を含むテスト
  test("0を含む", () => {
    expect(lcm(0, 5)).toBe(0);
    expect(lcm(4, 0)).toBe(0);
    expect(lcm(0, 0)).toBe(0);
  });

  // 負数を含むテスト
  test("負数を含む", () => {
    expect(lcm(-4, 5)).toBe(-20);
    expect(lcm(4, -5)).toBe(-20);
    expect(lcm(-4, -5)).toBe(20);
  });
});
