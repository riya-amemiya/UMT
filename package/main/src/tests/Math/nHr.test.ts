import { nHr } from "@/Math/nHr";
describe("nHr function", () => {
  // 正常系
  test("calculate nHr for n=5, r=2", () => {
    expect(nHr(5, 2)).toBe(15);
  });

  test("calculate nHr for n=3, r=3", () => {
    expect(nHr(3, 3)).toBe(10);
  });

  // 異常系
  test("return NaN when n or r is 0", () => {
    expect(nHr(0, 5)).toBeNaN();
    expect(nHr(5, 0)).toBeNaN();
  });

  test("return NaN when n or r is negative", () => {
    expect(nHr(-1, 5)).toBeNaN();
    expect(nHr(5, -1)).toBeNaN();
  });
});
