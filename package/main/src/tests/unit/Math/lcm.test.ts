import { lcm } from "@/Math/lcm";

describe("lcm関数のテスト", () => {
  // 正数同士のテスト
  test("正数の入力", () => {
    expect(lcm(4, 5)).toBe(20);
    expect(lcm(6, 8)).toBe(24);
    expect(lcm(10, 15)).toBe(30);
    expect(lcm(7, 11)).toBe(77);
  });

  // 0を含むテスト
  test("0を含む", () => {
    expect(lcm(0, 5)).toBe(0);
    expect(lcm(4, 0)).toBe(0);
    expect(lcm(0, 0)).toBe(0);
    expect(lcm(0, 10)).toBe(0);
    expect(lcm(7, 0)).toBe(0);
  });

  // 1を含むテスト
  test("1を含む", () => {
    expect(lcm(1, 5)).toBe(5);
    expect(lcm(4, 1)).toBe(4);
    expect(lcm(1, 1)).toBe(1);
    expect(lcm(1, 10)).toBe(10);
    expect(lcm(7, 1)).toBe(7);
  });

  // 大きな数値のテスト
  test("大きな数値", () => {
    expect(lcm(1000, 2000)).toBe(2000);
    expect(lcm(12345, 67890)).toBe(55873470);
  });
});
