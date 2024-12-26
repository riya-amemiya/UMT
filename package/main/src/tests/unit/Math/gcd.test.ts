import { gcd } from "@/Math/gcd";
describe("gcd function", () => {
  // 正の整数が２つの場合
  test("two positive integers", () => {
    expect(gcd(56, 48)).toBe(8);
  });

  // 正の整数が３つ以上の場合
  test("more than two positive integers", () => {
    expect(gcd(56, 48, 32)).toBe(8);
    expect(gcd(56, 48, 32, 24)).toBe(8);
  });

  // 0が含まれる場合
  test("contains zero", () => {
    expect(gcd(56, 0)).toBe(0);
    expect(gcd(0, 56)).toBe(0);
  });

  // 1が含まれる場合
  test("contains one", () => {
    expect(gcd(56, 1)).toBe(1);
    expect(gcd(1, 56)).toBe(1);
  });

  // 負の数が含まれる場合
  test("contains negative numbers", () => {
    expect(gcd(-56, 48)).toBe(8);
    expect(gcd(56, -48)).toBe(8);
    expect(gcd(-56, -48)).toBe(8);
    expect(gcd(-56, 48, -32)).toBe(8);
  });
});
