import { nCr } from "@/Math/nCr";
describe("nCr function", () => {
  // 正常系
  test("calculate nCr for n=5, r=2", () => {
    expect(nCr(5, 2)).toBe(10);
  });

  test("calculate nCr for n=10, r=4", () => {
    expect(nCr(10, 4)).toBe(210);
  });

  // 異常系
  test("return NaN when n or r is 0", () => {
    expect(nCr(0, 5)).toBeNaN();
    expect(nCr(5, 0)).toBeNaN();
  });

  test("return NaN when n < r", () => {
    expect(nCr(2, 5)).toBeNaN();
  });
});
