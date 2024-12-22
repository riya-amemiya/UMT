import { nPr } from "@/Math/nPr";
describe("nPr function", () => {
  // 正常系
  test("calculate nPr for n=5, r=2", () => {
    expect(nPr(5, 2)).toBe(20);
  });

  test("calculate nPr for n=10, r=4", () => {
    expect(nPr(10, 4)).toBe(5040);
  });

  // 異常系
  test("return NaN when n or r is 0", () => {
    expect(nPr(0, 5)).toBeNaN();
    expect(nPr(5, 0)).toBeNaN();
  });

  test("return NaN when n < r", () => {
    expect(nPr(2, 5)).toBeNaN();
  });
});
